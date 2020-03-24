use crate::{
	db::{
		models::{
			comment::Comment,
			deal::Deal,
			exercise::{Exercise, ExerciseBid},
		},
		mongo,
	},
	result::Result,
	server::auth,
};
use actix_web::{
	get, post,
	web::{self, Json},
};
use bridge_core as core;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/exercises").service(get_exercise_for_bid));
	cfg.service(
		web::scope("/exercise")
			.service(get_exercise_by_id)
			.service(get_bids_for_exercise)
			.service(make_bid)
			.service(comment),
	);
	cfg.service(web::scope("/bid").service(get_exercise_bid_by_id));
}

#[derive(Serialize)]
struct GetExerciseRes {
	deal: core::Deal,
	exercise_id: String,
	bids: Vec<String>,
	comments: Vec<Comment>,
}

#[get("/bid")]
async fn get_exercise_for_bid(mc: mongo::Client, tok: auth::Token) -> Result<Json<GetExerciseRes>> {
	let user = tok.user;
	let ex = Exercise::get_unbid_or_create(mc.clone(), user.id)?;
	let deal = Deal::get_by_id(mc.clone(), ex.deal_id)?;
	let res = GetExerciseRes {
		deal: deal.deal,
		exercise_id: ex.id.clone().to_string(),
		bids: ex.bids.bids().iter().map(|b| format!("{}", b)).collect(),
		comments: Comment::get_by_exercise_id(mc.clone(), ex.id)?,
	};
	Ok(Json(res))
}

#[get("/{ex_id}")]
async fn get_exercise_by_id(
	mc: mongo::Client,
	ex_id: web::Path<String>,
) -> Result<Json<GetExerciseRes>> {
	let ex_id = ObjectId::with_string(&ex_id.into_inner())?;
	let ex = Exercise::get_by_id(mc.clone(), ex_id.clone())?;
	let deal = Deal::get_by_id(mc.clone(), ex.deal_id)?;
	let res = GetExerciseRes {
		deal: deal.deal,
		exercise_id: ex_id.to_string(),
		bids: ex.bids.bids().iter().map(|b| format!("{}", b)).collect(),
		comments: Comment::get_by_exercise_id(mc.clone(), ex_id)?,
	};
	Ok(Json(res))
}

#[derive(Serialize)]
struct GetExerciseBidRes {
	exercise_bid_id: String,
	exercise_id: String,
	user_id: String,
	next_bid: String,
}

impl GetExerciseBidRes {
	fn new(mc: mongo::Client, id: ObjectId) -> Result<GetExerciseBidRes> {
		let ex_bid = ExerciseBid::get_by_id(mc, id)?;
		let res = GetExerciseBidRes {
			exercise_bid_id: ex_bid.id.to_string(),
			exercise_id: ex_bid.exercise_id.to_string(),
			user_id: ex_bid.user_id.to_string(),
			next_bid: format!("{}", ex_bid.bid),
		};
		Ok(res)
	}
}

#[get("/{ex_bid_id}")]
async fn get_exercise_bid_by_id(
	mc: mongo::Client,
	ex_bid_id: web::Path<String>,
) -> Result<Json<GetExerciseBidRes>> {
	let ex_bid_id = ObjectId::with_string(&ex_bid_id.into_inner())?;
	let res = GetExerciseBidRes::new(mc, ex_bid_id)?;
	Ok(Json(res))
}

#[derive(Serialize)]
struct GetExerciseBidsRes {
	bids: Vec<GetExerciseBidRes>,
}

#[get("/{ex_id}/bids")]
async fn get_bids_for_exercise(
	mc: mongo::Client,
	ex_id: web::Path<String>,
) -> Result<Json<GetExerciseBidsRes>> {
	let ex_id = ObjectId::with_string(&ex_id.into_inner())?;
	let ex = Exercise::get_by_id(mc.clone(), ex_id)?;
	let bids = ExerciseBid::get_by_exercise_id(mc.clone(), ex.id)?;
	let bids = bids
		.into_iter()
		.map(|b| GetExerciseBidRes::new(mc.clone(), b.id))
		.collect::<Result<Vec<GetExerciseBidRes>>>()?;
	let res = GetExerciseBidsRes { bids };
	Ok(Json(res))
}

#[derive(Deserialize)]
struct MakeBidReq {
	bid: String,
}

#[derive(Serialize)]
struct MakeBidRes {
	exercise_bid_id: String,
}

#[post("/{ex_id}/bid")]
async fn make_bid(
	mc: mongo::Client,
	tok: auth::Token,
	body: Json<MakeBidReq>,
	ex_id: web::Path<String>,
) -> Result<Json<MakeBidRes>> {
	let user = tok.user;
	let body = body.0;
	let ex_id = ObjectId::with_string(&ex_id.into_inner())?;

	let ex = Exercise::get_by_id(mc.clone(), ex_id)?;
	let bid = core::Bid::parse(&body.bid)?;
	let ex_bid = ex.insert_bid(mc.clone(), user.id, bid)?;
	ex.generate_followup(mc.clone(), &bid)?;

	Ok(Json(MakeBidRes {
		exercise_bid_id: ex_bid.id.to_string(),
	}))
}

#[derive(Deserialize)]
struct CommentReq {
	text: String,
}

#[derive(Serialize)]
struct CommentRes {}

#[post("/{ex_id}/comment")]
async fn comment(
	mc: mongo::Client,
	tok: auth::Token,
	body: Json<CommentReq>,
	ex_id: web::Path<String>,
) -> Result<Json<CommentRes>> {
	let user = tok.user;
	let body = body.0;
	let ex_id = ObjectId::with_string(&ex_id.into_inner())?;

	let ex = Exercise::get_by_id(mc.clone(), ex_id)?;
	ex.add_comment(mc.clone(), user.id.clone(), body.text)?;

	Ok(Json(CommentRes {}))
}
