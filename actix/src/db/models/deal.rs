use crate::{
	db::mongo,
	result::{Error, Result},
};
use bridge_core as core;
use bson::{self, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deal {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub deal: core::Deal,
}

impl Deal {
	pub fn generate(mc: mongo::Client) -> Result<Self> {
		let deal = core::Deal::random();
		Self::insert(mc, deal)
	}

	pub fn generate_first_seat_one_nt_opener(mc: mongo::Client) -> Result<Self> {
		let deal = core::Deal::first_seat_one_nt_opener();
		Self::insert(mc, deal)
	}

	fn insert(mc: mongo::Client, deal: core::Deal) -> Result<Self>
	{
		let deal = Deal {
			id: ObjectId::new()?,
			deal,
		};
		let ser = bson::to_bson(&deal)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("deals")
				.insert_one(doc, None)?;
		} else {
			unreachable!("a deal should never deserialize into a non-doc bson value");
		}
		Ok(deal)
	}

	pub fn get_by_id(mc: mongo::Client, id: ObjectId) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("deals")
			.find_one(doc! {"_id": id}, None)?
			.ok_or(Error::DealNotFound)?;
		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}
}
