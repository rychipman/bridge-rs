use crate::{
	db::{
		models::{comment::Comment, deal::Deal},
		mongo,
	},
	result::{Error, Result},
};
use bridge_core as core;
use bson::{self, doc, oid::ObjectId, UtcDateTime};
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub deal_id: ObjectId,
	pub bids: core::BidSequence,
	pub parent_id: Option<ObjectId>,
	pub created: UtcDateTime,
}

impl Exercise {
	fn insert(&self, mc: mongo::Client) -> Result<()> {
		let ser = bson::to_bson(self)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("exercises")
				.insert_one(doc, None)?;
		} else {
			unreachable!("an exercise should never deserialize into a non-doc bson value");
		}
		Ok(())
	}

	pub fn generate_with_deal(mc: mongo::Client, deal_id: ObjectId) -> Result<Self> {
		let ex = Exercise {
			id: ObjectId::new()?,
			deal_id: deal_id,
			bids: core::BidSequence::empty(),
			parent_id: None,
			created: UtcDateTime(Utc::now()),
		};
		ex.insert(mc)?;
		Ok(ex)
	}

	pub fn get_by_id(mc: mongo::Client, id: ObjectId) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("exercises")
			.find_one(doc! {"_id": id}, None)?
			.ok_or(Error::ExerciseNotFound)?;
		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}

	pub fn generate_followup(&self, mc: mongo::Client, bid: &core::Bid) -> Result<Option<Self>> {
		let followup = Exercise {
			id: ObjectId::new()?,
			deal_id: self.deal_id.clone(),
			bids: self.bids.with_continuation(*bid)?,
			parent_id: Some(self.id.clone()),
			created: UtcDateTime(Utc::now()),
		};
		if followup.bids.is_finished() {
			Ok(None)
		} else {
			followup.insert(mc)?;
			Ok(Some(followup))
		}
	}

	pub fn get_unbid(mc: mongo::Client, user_id: ObjectId) -> Result<Vec<Self>> {
		// get deal_id associated with last n exercises bid by the current user
		let lookback = 5;
		let recent_exercises_pipeline = vec![
			doc! {"$sort": {"_id": -1}},
			doc! {"$match": {"user_id": user_id.clone()}},
			doc! {"$limit": lookback},
			doc! {"$lookup": {
				"from": "exercises",
				"as": "exercise",
				"localField": "exercise_id",
				"foreignField": "_id",
			}},
			doc! {"$unwind": "$exercise"},
			doc! {"$replaceRoot": {"newRoot": "$exercise"}},
		];
		let mut recent_deal_ids = mc
			.database("bridge")
			.collection("exercise_bids")
			.aggregate(recent_exercises_pipeline, None)?
			.map(|doc| {
				let ex: Self = bson::from_bson(bson::Bson::Document(doc?))?;
				Ok(ex.deal_id)
			})
			.collect::<Result<Vec<ObjectId>>>()?;
		recent_deal_ids.sort();
		recent_deal_ids.dedup();

		let pipeline = vec![
			doc! {"$match": {"deal_id": {"$nin": recent_deal_ids}}},
			doc! {"$lookup": {
				"from": "exercise_bids",
				"as": "bids",
				"let": { "ex_id": "$_id" },
				"pipeline": [
					{"$match": {"user_id": user_id}},
					{"$match": {"$expr": {"$eq": ["$exercise_id", "$$ex_id"]}}},
				],
			}},
			doc! {"$addFields": {
				"is_unbid": {"$eq": [0, {"$size": "$bids"}]},
			}},
			doc! {"$match": {
				"is_unbid": true,
			}},
		];
		let res: Vec<Self> = mc
			.database("bridge")
			.collection("exercises")
			.aggregate(pipeline, None)?
			.map(|doc| {
				let ex: Self = bson::from_bson(bson::Bson::Document(doc?))?;
				Ok(ex)
			})
			.collect::<Result<Vec<Self>>>()?;
		println!("found {} unbid exercises: {:?}", res.len(), res);
		Ok(res)
	}

	pub fn get_unbid_or_create(mc: mongo::Client, user_id: ObjectId) -> Result<Self> {
		let mut unbid = Self::get_unbid(mc.clone(), user_id)?;
		if unbid.len() > 0 {
			return Ok(unbid.remove(0));
		}
		let deal = Deal::generate(mc.clone())?;
		let ex = Self::generate_with_deal(mc, deal.id)?;
		Ok(ex)
	}

	pub fn insert_bid(
		&self,
		mc: mongo::Client,
		user_id: ObjectId,
		bid: core::Bid,
	) -> Result<ExerciseBid> {
		self.bids.validate_continuation(bid)?;
		let ex_bid = ExerciseBid::create(mc, self.id.clone(), user_id, bid)?;
		Ok(ex_bid)
	}

	pub fn add_comment(&self, mc: mongo::Client, user_id: ObjectId, text: String) -> Result<()> {
		Comment::create(mc, user_id, self.id.clone(), text)?;
		Ok(())
	}
}

#[derive(Serialize, Deserialize)]
pub struct ExerciseBid {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub exercise_id: ObjectId,
	pub user_id: ObjectId,
	pub bid: core::Bid,
	pub created: UtcDateTime,
}

impl ExerciseBid {
	pub fn create(
		mc: mongo::Client,
		ex_id: ObjectId,
		uid: ObjectId,
		bid: core::Bid,
	) -> Result<Self> {
		let ex_bid = ExerciseBid {
			id: ObjectId::new()?,
			exercise_id: ex_id,
			user_id: uid,
			bid,
			created: UtcDateTime(Utc::now()),
		};
		let ser = bson::to_bson(&ex_bid)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("exercise_bids")
				.insert_one(doc, None)?;
		} else {
			unreachable!("an exercise bid should never deserialize into a non-doc bson value");
		}
		Ok(ex_bid)
	}

	pub fn get_by_id(mc: mongo::Client, id: ObjectId) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("exercise_bids")
			.find_one(doc! {"_id": id}, None)?
			.ok_or(Error::ExerciseBidNotFound)?;
		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}

	pub fn get_by_exercise_id(mc: mongo::Client, exercise_id: ObjectId) -> Result<Vec<Self>> {
		mc.database("bridge")
			.collection("exercise_bids")
			.find(doc! {"exercise_id": exercise_id}, None)?
			.map(|doc| {
				let ex_bid: Self = bson::from_bson(bson::Bson::Document(doc?))?;
				Ok(ex_bid)
			})
			.collect()
	}
}
