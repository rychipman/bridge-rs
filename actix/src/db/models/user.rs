use crate::{
	db::models::session::{Cred, Session},
	db::mongo,
	result::{Error, Result},
};
use bson::{doc, oid::ObjectId, UtcDateTime};
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub email: String,
	pub pw_hash: String,
	pub created: UtcDateTime,
	pub last_active: UtcDateTime,
}

impl User {
	pub fn get_all(mc: mongo::Client) -> Result<Vec<Self>> {
		mc.database("bridge")
			.collection("users")
			.find(None, None)?
			.map(|doc| {
				let user: Self = bson::from_bson(bson::Bson::Document(doc?))?;
				Ok(user)
			})
			.collect()
	}

	pub fn get_by_id(mc: mongo::Client, id: ObjectId) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("users")
			.find_one(doc! {"_id": id}, None)?
			.ok_or(Error::UserNotFound)?;
		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}

	pub fn get_by_email(mc: mongo::Client, email: &str) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("users")
			.find_one(doc! {"email": email}, None)?
			.ok_or(Error::UserNotFound)?;
		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}

	pub fn get_by_token(mc: mongo::Client, token: String) -> Result<Self> {
		let pipeline = vec![
			doc! {"$match": {"cred.Token": token}},
			doc! {"$lookup": {
				"from": "users",
				"localField": "user_id",
				"foreignField": "_id",
				"as": "user",
			}},
			doc! {"$unwind": "$user"},
			doc! {"$replaceRoot": { "newRoot": "$user" }},
		];
		let doc_opt = mc
			.database("bridge")
			.collection("sessions")
			.aggregate(pipeline, None)?
			.next();

		let doc = match doc_opt {
			Some(doc) => Ok(doc?),
			None => Err(Error::InvalidSession),
		}?;

		Ok(bson::from_bson(bson::Bson::Document(doc))?)
	}

	pub fn register(mc: mongo::Client, email: &str, pwd: &str) -> Result<Self> {
		match Self::get_by_email(mc.clone(), email) {
			Ok(_) => Err(Error::UserAlreadyExists),
			Err(Error::UserNotFound) => Ok(()),
			Err(e) => Err(e),
		}?;

		let pw_hash = Self::hash_password(pwd)?;

		let user = User {
			id: ObjectId::new()?,
			email: email.to_string(),
			pw_hash,
			created: UtcDateTime(Utc::now()),
			last_active: UtcDateTime(Utc::now()),
		};
		let ser = bson::to_bson(&user)?;
		if let bson::Bson::Document(doc) = ser {
			mc.database("bridge")
				.collection("users")
				.insert_one(doc, None)?;
		} else {
			unreachable!("a deal should never deserialize into a non-doc bson value");
		}
		Ok(user)
	}

	pub fn login(mc: mongo::Client, email: &str, pwd: &str) -> Result<(Self, Cred)> {
		let user = Self::get_by_email(mc.clone(), email)?;
		match user.verify_password(pwd) {
			Ok(true) => Ok(()),
			Ok(false) => Err(Error::IncorrectPassword),
			Err(e) => Err(e),
		}?;
		let cred = Session::new_token(mc, user.id.clone())?;
		Ok((user, cred))
	}

	fn hash_password(pwd: &str) -> Result<String> {
		let hash = bcrypt::hash(pwd, bcrypt::DEFAULT_COST)?;
		Ok(hash)
	}

	fn verify_password(&self, pwd: &str) -> Result<bool> {
		let matches = bcrypt::verify(pwd, &self.pw_hash)?;
		Ok(matches)
	}

	pub fn update_last_active(&self, mc: mongo::Client) -> Result<()> {
		mc.database("bridge")
			.collection("users")
			.find_one_and_update(
				doc! {"_id": self.id.clone()},
				doc! {"$currentDate": {"last_active": true}},
				None,
			)?;
		Ok(())
	}
}
