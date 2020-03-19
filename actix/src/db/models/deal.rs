use crate::result::Result;
use bridge_core as core;
use bson::oid::ObjectId;
use mongodb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deal {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub deal: core::Deal,
}

impl Deal {
	pub fn generate(mc: mongodb::Client) -> Result<Self> {
		let deal = Deal {
			id: ObjectId::new()?,
			deal: core::Deal::random(),
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
}
