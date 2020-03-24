use crate::{db::mongo, result::Result};
use bson::{oid::ObjectId, UtcDateTime};
use chrono::offset::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::iter;

#[derive(Serialize, Deserialize)]
pub struct Session {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub user_id: ObjectId,
	pub cred: Cred,
	pub created: UtcDateTime,
}

impl Session {
	pub fn new_token(mc: mongo::Client, user_id: ObjectId) -> Result<Cred> {
		let cred = Cred::new_token();
		let session = Session {
			id: ObjectId::new()?,
			user_id,
			cred: cred.clone(),
			created: UtcDateTime(Utc::now()),
		};

		let ser = bson::to_bson(&session)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("sessions")
				.insert_one(doc, None)?;
		} else {
			unreachable!("a session should never deserialize into a non-doc bson value");
		}
		Ok(cred)
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Cred {
	Token(String),
}

impl Cred {
	fn new_token() -> Self {
		Cred::Token(Self::rand_string())
	}

	fn rand_string() -> String {
		let mut rng = rand::thread_rng();
		iter::repeat(())
			.map(|()| rng.sample(rand::distributions::Alphanumeric))
			.take(24)
			.collect()
	}
}
