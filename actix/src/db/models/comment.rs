use crate::{db::mongo, result::Result};
use bson::{self, UtcDateTime, oid::ObjectId, doc};
use chrono::offset::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub user_id: ObjectId,
	pub exercise_id: ObjectId,
	pub text: String,
	pub created: UtcDateTime,
}

impl Comment {
	fn insert(&self, mc: mongo::Client) -> Result<()> {
		let ser = bson::to_bson(self)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("comments")
				.insert_one(doc, None)?;
		} else {
			unreachable!("a comment should never deserialize into a non-doc bson value");
		}
		Ok(())
	}

	pub fn create(
		mc: mongo::Client,
		user_id: ObjectId,
		exercise_id: ObjectId,
		text: String,
	) -> Result<Self> {
		let cmt = Self {
			id: ObjectId::new()?,
			user_id,
			exercise_id,
			text,
			created: UtcDateTime(Utc::now()),
		};
		cmt.insert(mc)?;
		Ok(cmt)
	}

	pub fn get_by_exercise_id(mc: mongo::Client, exercise_id: ObjectId) -> Result<Vec<Self>> {
		mc.database("bridge")
    		.collection("comments")
    		.find(doc! {"exercise_id": exercise_id}, None)?
    		.map(|doc| {
				let comment: Self = bson::from_bson(bson::Bson::Document(doc?))?;
				Ok(comment)
			})
    		.collect()
	}
}
