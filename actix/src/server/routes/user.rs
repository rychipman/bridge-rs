use crate::{
	db::{models::user::User, mongo},
	result::Result,
};
use actix_web::{
	get,
	web::{self, Json},
};
use bson::oid::ObjectId;
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(get_users).service(get_user_by_id);
}

#[derive(Serialize)]
struct GetUsersRes {
	users: Vec<User>,
}

#[get("/users")]
async fn get_users(mc: mongo::Client) -> Result<Json<GetUsersRes>> {
	let res = GetUsersRes {
		users: User::get_all(mc)?,
	};
	Ok(Json(res))
}

#[derive(Serialize)]
struct GetUserRes {
	user: User,
}

#[get("/user/{id}")]
async fn get_user_by_id(mc: mongo::Client, uid: web::Path<String>) -> Result<Json<GetUserRes>> {
	let uid = ObjectId::with_string(&uid.into_inner())?;
	let res = GetUserRes {
		user: User::get_by_id(mc, uid)?,
	};
	Ok(Json(res))
}
