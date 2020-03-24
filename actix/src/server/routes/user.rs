use crate::{
	db::{models::user::User, mongo},
	result::Result,
};
use actix_web::{
	get,
	web::{self, Json},
};
use bson::oid::ObjectId;
use chrono::{offset::Utc, DateTime};
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(get_users).service(get_user_by_id);
}

#[derive(Serialize)]
struct UserRes {
	id: String,
	email: String,
	last_active: DateTime<Utc>,
}

impl From<User> for UserRes {
	fn from(model: User) -> Self {
		UserRes {
			id: model.id.to_hex(),
			email: model.email,
			last_active: *model.last_active,
		}
	}
}

#[derive(Serialize)]
struct GetUsersRes {
	users: Vec<UserRes>,
}

#[get("/users")]
async fn get_users(mc: mongo::Client) -> Result<Json<GetUsersRes>> {
	let res = GetUsersRes {
		users: User::get_all(mc)?.into_iter().map(UserRes::from).collect(),
	};
	Ok(Json(res))
}

#[derive(Serialize)]
struct GetUserRes {
	user: UserRes,
}

#[get("/user/{id}")]
async fn get_user_by_id(mc: mongo::Client, uid: web::Path<String>) -> Result<Json<GetUserRes>> {
	let uid = ObjectId::with_string(&uid.into_inner())?;
	let res = GetUserRes {
		user: User::get_by_id(mc, uid)?.into(),
	};
	Ok(Json(res))
}
