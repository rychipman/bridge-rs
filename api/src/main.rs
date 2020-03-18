#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bcrypt;
extern crate bridge;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod models;
mod schema;

use bridge::game::Bid;
use models::{
	auth::{Token, User, UserInsert},
	bridge::{get_random_conflicting_exercise, Comment, Deal, Exercise, ExerciseBid},
};
use rocket::{http::Status, request::FromRequest, Outcome, Request};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Response<T> {
	success: bool,
	data: Option<T>,
	error: Option<String>,
}

impl<T> Response<T> {
	fn ok(data: T) -> Response<T> {
		Response {
			success: true,
			data: Some(data),
			error: None,
		}
	}

	fn err(msg: String) -> Response<T> {
		Response {
			success: false,
			data: None,
			error: Some(msg),
		}
	}
}

struct ApiToken(User);
impl<'a, 'r> FromRequest<'a, 'r> for ApiToken {
	type Error = String;
	fn from_request(req: &'a Request<'r>) -> Outcome<Self, (Status, String), ()> {
		let keys: Vec<_> = req.headers().get("Authorization").collect();
		if keys.len() != 1 {
			return Outcome::Failure((Status::Unauthorized, String::from("Missing Token")));
		}

		let words: Vec<String> = keys[0]
			.to_string()
			.split_whitespace()
			.map(String::from)
			.collect();
		if words.len() != 2 || words[0] != "bearer" {
			return Outcome::Failure((Status::Unauthorized, String::from("Malformed Token")));
		}

		let conn: DbConn = req.guard().expect("failed to get db conn in req guard");

		let token = words[1].clone().to_string();
		let user =
			User::by_token(&conn, token).expect("failed to get user from token in req guard");
		user.register_activity(&conn)
			.expect("failed to register activity for user");

		Outcome::Success(ApiToken(user))
	}
}

#[derive(Deserialize)]
struct LoginReq {
	email: String,
	password: String,
}

#[derive(Serialize)]
struct LoginRes {
	email: String,
	token: String,
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(conn: DbConn, data: Json<LoginReq>) -> Json<Response<LoginRes>> {
	let res = match User::by_email(&conn, &data.0.email) {
		Ok(ref user) if !user.verify_password(&data.0.password) => {
			Response::err("incorrect password".into())
		}
		Ok(user) => {
			let payload = LoginRes {
				email: data.email.clone(),
				token: Token::new(&conn, user.id)
					.expect("failed to insert new token")
					.token,
			};
			Response::ok(payload)
		}
		Err(e) => Response::err(e.to_string()),
	};
	Json(res)
}

#[derive(Deserialize)]
struct RegisterReq {
	email: String,
	password: String,
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(conn: DbConn, data: Json<RegisterReq>) -> Json<Response<()>> {
	let user = UserInsert::new(data.0.email, data.0.password);
	let res = match user.insert(&conn) {
		Ok(_) => Response::ok(()),
		Err(e) => Response::err(e.to_string()),
	};
	Json(res)
}

#[derive(Serialize)]
struct ExerciseRes {
	deal: Deal,
	exercise_id: i32,
	bids: Vec<String>,
	comments: Vec<Comment>,
}

#[get("/exercises/conflict")]
fn get_exercise_with_conflict(conn: DbConn, auth: ApiToken) -> Json<Response<ExerciseRes>> {
	let user = auth.0;
	let ex = get_random_conflicting_exercise(&conn, &user)
		.expect("failed to get exercise with conflict")
		.pop()
		.expect("no exercises had conflicts");
	let deal = Deal::by_id(&conn, ex.deal_id).expect("failed to get deal for exercise");
	let comments = ex
		.get_comments(&conn)
		.expect("failed to get comments on exercise");
	let res = ExerciseRes {
		deal,
		exercise_id: ex.id,
		bids: ex.bids.bids().iter().map(|b| format!("{}", b)).collect(),
		comments,
	};
	Json(Response::ok(res))
}

#[get("/exercises/bid")]
fn get_exercise_for_bid(conn: DbConn, auth: ApiToken) -> Json<Response<ExerciseRes>> {
	let ex = Exercise::get_unbid_or_create(&conn, &auth.0).expect("failed to get exercise");
	let deal = Deal::by_id(&conn, ex.deal_id).expect("failed to get deal for exercise");
	let comments = ex
		.get_comments(&conn)
		.expect("failed to get comments on exercise");
	let res = ExerciseRes {
		deal,
		exercise_id: ex.id,
		bids: ex.bids.bids().iter().map(|b| format!("{}", b)).collect(),
		comments,
	};
	Json(Response::ok(res))
}

#[get("/exercise/<ex_id>")]
fn get_exercise_by_id(conn: DbConn, ex_id: i32) -> Json<Response<ExerciseRes>> {
	let ex = Exercise::by_id(&conn, ex_id).expect("failed to get exercise by id");
	let deal = Deal::by_id(&conn, ex.deal_id).expect("failed to get deal for exercise");
	let comments = ex
		.get_comments(&conn)
		.expect("failed to get comments on exercise");
	let res = ExerciseRes {
		deal,
		exercise_id: ex.id,
		bids: ex.bids.bids().iter().map(|b| format!("{}", b)).collect(),
		comments,
	};
	Json(Response::ok(res))
}

#[derive(Serialize)]
struct ExerciseBidRes {
	exercise_bid_id: i32,
	exercise_id: i32,
	user_id: i32,
	next_bid: String,
}

impl ExerciseBidRes {
	fn new(conn: &DbConn, id: i32) -> ExerciseBidRes {
		let ex_bid = ExerciseBid::by_id(conn, id).expect("failed to get exercise bid by id");
		ExerciseBidRes {
			exercise_bid_id: ex_bid.id,
			exercise_id: ex_bid.exercise_id,
			user_id: ex_bid.user_id,
			next_bid: format!("{}", ex_bid.bid),
		}
	}
}

#[get("/bid/<ex_bid_id>")]
fn get_exercise_bid_by_id(conn: DbConn, ex_bid_id: i32) -> Json<Response<ExerciseBidRes>> {
	Json(Response::ok(ExerciseBidRes::new(&conn, ex_bid_id)))
}

#[derive(Serialize)]
struct ExerciseBidsRes {
	bids: Vec<ExerciseBidRes>,
}

#[get("/exercise/<ex_id>/bids")]
fn get_bids_for_exercise(conn: DbConn, ex_id: i32) -> Json<Response<ExerciseBidsRes>> {
	let exercise = Exercise::by_id(&conn, ex_id).expect("failed to find exercise for provided id");
	let bids =
		ExerciseBid::by_exercise_id(&conn, exercise.id).expect("failed to find bids for exercise");
	let bids = bids
		.iter()
		.map(|b| ExerciseBidRes::new(&conn, b.id))
		.collect();
	let res = ExerciseBidsRes { bids };
	Json(Response::ok(res))
}

#[derive(Deserialize)]
struct MakeBidReq {
	bid: String,
}

#[derive(Serialize)]
struct MakeBidRes {
	exercise_bid_id: i32,
}

#[post("/exercise/<id>/bid", format = "application/json", data = "<data>")]
fn make_bid(
	conn: DbConn,
	auth: ApiToken,
	id: i32,
	data: Json<MakeBidReq>,
) -> Json<Response<MakeBidRes>> {
	let user = auth.0;
	let ex = Exercise::by_id(&conn, id).expect("failed to find exercise for provided id");
	let bid = Bid::parse(&data.0.bid).expect("failed to parse bid from provided string");
	let ex_bid = ex
		.insert_bid(&conn, user.id, &bid)
		.expect("failed to insert new bid for exercise");
	ex_bid
		.create_followup_exercise(&conn)
		.expect("failed to create followup exercise");
	let res = MakeBidRes {
		exercise_bid_id: ex_bid.id,
	};
	Json(Response::ok(res))
}

//#[derive(Serialize)]
//struct ReviewRes {
//    exercise: Exercise,
//    bids: Vec<ExerciseBid>,
//}
//
//#[get("/exercise/<id>/review")]
//fn review_exercise(conn: DbConn, _auth: ApiToken, id: i32) -> Json<Response<ReviewRes>> {
//    let exercise = Exercise::by_id(&conn, id).expect("failed to find exercise for provided id");
//    let bids =
//        ExerciseBid::by_exercise_id(&conn, exercise.id).expect("failed to find bids for exercise");
//    let res = ReviewRes { exercise, bids };
//    Json(Response::ok(res))
//}

#[derive(Serialize)]
struct UsersRes {
	users: Vec<User>,
}

#[get("/users")]
fn get_users(conn: DbConn) -> Json<Response<UsersRes>> {
	let users = User::all(&conn).expect("failed to get users");
	let res = UsersRes { users };
	Json(Response::ok(res))
}

#[derive(Serialize)]
struct UserRes {
	user: User,
}

#[get("/user/<uid>")]
fn get_user_by_id(conn: DbConn, uid: i32) -> Json<Response<UserRes>> {
	let user = User::by_id(&conn, uid).expect("failed to get user by id");
	let res = UserRes { user };
	Json(Response::ok(res))
}

#[derive(Deserialize)]
struct CommentReq {
	text: String,
}

#[post(
	"/exercise/<ex_id>/comment",
	format = "application/json",
	data = "<data>"
)]
fn comment(conn: DbConn, auth: ApiToken, ex_id: i32, data: Json<CommentReq>) -> Json<Response<()>> {
	let user = auth.0;
	let ex = Exercise::by_id(&conn, ex_id).expect("failed to get exercise by id");
	ex.create_comment(&conn, &user, data.0.text)
		.expect("failed to create comment");
	Json(Response::ok(()))
}

#[database("sqlite_bridgeskills")]
pub struct DbConn(diesel::SqliteConnection);

fn main() {
	rocket::ignite()
		.mount(
			"/",
			routes![
				login,
				register,
				get_exercise_with_conflict,
				get_exercise_for_bid,
				get_exercise_by_id,
				get_exercise_bid_by_id,
				get_bids_for_exercise,
				make_bid,
				//review_exercise,
				get_users,
				get_user_by_id,
				comment
			],
		)
		.attach(DbConn::fairing())
		.launch();
}
