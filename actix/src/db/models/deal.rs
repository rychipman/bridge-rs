use crate::{
	db::mongo,
	result::{Error, Result},
};
use bridge_core as core;
use bson::{self, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::ops::Fn;

#[derive(Serialize, Deserialize)]
pub struct Deal {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub deal: core::Deal,
}

impl Deal {
	pub fn generate(mc: mongo::Client) -> Result<Self> {
		Self::generate_with_constructor(mc, core::Deal::random)
	}

	pub fn generate_first_seat_one_nt_opener(mc: mongo::Client) -> Result<Self> {
		Self::generate_with_constructor(mc, core::Deal::first_seat_one_nt_opener)
	}

	fn generate_with_constructor<F>(mc: mongo::Client, constr: F) -> Result<Self>
		where F: Fn() -> core::Deal,
	{
		let deal = Deal {
			id: ObjectId::new()?,
			deal: constr(),
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
