use crate::{db::mongo, models::user::User, result::Result};
use actix_web::{
	post,
	web::{self, Json},
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
async fn register(body: Json<RegisterReq>, mc: mongo::Client) -> Result<Json<RegisterRes>> {
	let user = User::register(mc, &body.0.email)?;
	Ok(Json(RegisterRes { email: user.email }))
}
