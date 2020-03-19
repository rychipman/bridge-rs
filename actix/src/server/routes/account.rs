use crate::{
	db::{
		models::{session::Cred, user::User},
		mongo,
	},
	result::Result,
};
use actix_web::{
	post,
	web::{self, Json},
};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/account").service(register).service(login));
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
	let body = body.0;
	let user = User::register(mc, &body.email, &body.password)?;
	Ok(Json(RegisterRes { email: user.email }))
}

#[derive(Deserialize)]
struct LoginReq {
	email: String,
	password: String,
}

#[derive(Serialize)]
struct LoginRes {
	token: String,
}

#[post("/login")]
async fn login(body: Json<LoginReq>, mc: mongo::Client) -> Result<Json<LoginRes>> {
	let body = body.0;
	let (_user, cred) = User::login(mc, &body.email, &body.password)?;
	let token = match cred {
		Cred::Token(tok) => tok,
	};
	Ok(Json(LoginRes { token }))
}
