use crate::{models::user::User, result::Result, server::MongoClient};
use actix_web::{
	post,
	web::{self, Json},
	Responder,
};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/account").service(register));
}

#[derive(Deserialize)]
struct RegisterReq {
	email: String,
	password: String,
}

#[derive(Serialize)]
struct RegisterRes {
	email: String,
}

#[post("/register")]
async fn register(
	body: Json<RegisterReq>,
	mongo: web::Data<mongodb::Client>,
) -> Result<Json<RegisterRes>> {
	let user = User::register(MongoClient(mongo), &body.0.email)?;
	Ok(Json(RegisterRes { email: user.email }))
}
