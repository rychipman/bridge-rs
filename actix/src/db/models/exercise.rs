use crate::result::Result;
use bridge_core as core;
use bson::oid::ObjectId;
use mongodb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Exercise {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub deal_id: ObjectId,
	pub bids: core::BidSequence,
}

impl Exercise {
	pub fn generate_with_deal(mc: mongodb::Client, deal_id: ObjectId) -> Result<Self> {
		let ex = Exercise {
			id: ObjectId::new()?,
			deal_id: deal_id,
			bids: core::BidSequence::empty(),
		};
		let ser = bson::to_bson(&ex)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("exercises")
				.insert_one(doc, None)?;
		} else {
			unreachable!("an exercise should never deserialize into a non-doc bson value");
		}
		Ok(ex)
	}
}

#[derive(Serialize, Deserialize)]
pub struct ExerciseBid {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub exercise_id: ObjectId,
	pub bid: core::Bid,
}
