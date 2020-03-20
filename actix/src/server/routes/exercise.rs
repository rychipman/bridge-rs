use crate::{db::mongo, result::Result};
use actix_web::{
	get, post,
	web::{self, Json},
};
use bridge_core as core;
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/exercises").service(get_exercise_for_bid));
	cfg.service(
		web::scope("/exercise")
			.service(get_exercise_by_id)
			.service(get_bids_for_exercise)
			.service(make_bid),
	);
	cfg.service(web::scope("/bid").service(get_exercise_bid_by_id));
}

#[derive(Serialize)]
struct GetExerciseRes {
	deal: core::Deal,
	exercise_id: String,
	bids: Vec<String>,
	//comments: Vec<Comment>,
}

#[get("/bid")]
async fn get_exercise_for_bid(mc: mongo::Client) -> Result<Json<GetExerciseRes>> {
	unimplemented!()
}

#[get("/<ex_id>")]
async fn get_exercise_by_id(mc: mongo::Client) -> Result<Json<GetExerciseRes>> {
	unimplemented!()
}

#[derive(Serialize)]
struct GetExerciseBidRes {
	exercise_bid_id: String,
	exercise_id: String,
	user_id: String,
	next_bid: String,
}

#[get("/<ex_bid_id>")]
async fn get_exercise_bid_by_id(mc: mongo::Client) -> Result<Json<GetExerciseBidRes>> {
	unimplemented!()
}

#[derive(Serialize)]
struct GetExerciseBidsRes {
	bids: Vec<GetExerciseBidRes>,
}

#[get("/<ex_id>/bids")]
async fn get_bids_for_exercise(mc: mongo::Client) -> Result<Json<GetExerciseBidsRes>> {
	unimplemented!()
}

#[derive(Deserialize)]
struct MakeBidReq {
	bid: String,
}

#[derive(Serialize)]
struct MakeBidRes {
	exercise_bid_id: String,
}

#[post("/<ex_id>/bid")]
async fn make_bid(body: Json<MakeBidReq>, mc: mongo::Client) -> Result<Json<MakeBidRes>> {
	unimplemented!()
}

/*
#[post("/register")]
async fn register(body: Json<RegisterReq>, mc: mongo::Client) -> Result<Json<RegisterRes>> {
	let body = body.0;
	let user = User::register(mc, &body.email, &body.password)?;
	Ok(Json(RegisterRes { email: user.email }))
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
*/
