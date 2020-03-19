use crate::{
	db::mongo,
	result::{Error, Result},
};
use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub email: String,
	pub pw_hash: String,
}

impl User {
	pub fn get_by_email(mc: mongo::Client, email: &str) -> Result<Self> {
		let doc = mc
			.database("bridge")
			.collection("users")
			.find_one(doc! {"email": email}, None)?
			.ok_or(Error::UserNotFound)?;
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

	fn hash_password(pwd: &str) -> Result<String> {
		let hash = bcrypt::hash(pwd, bcrypt::DEFAULT_COST)?;
		Ok(hash)
	}

	pub fn verify_password(&self, pwd: &str) -> Result<bool> {
		let matches = bcrypt::verify(pwd, &self.pw_hash)?;
		Ok(matches)
	}
}
