use super::auth::User;
use bridge::game::{Bid, BidSequence, Deck, Hand, Seat, Vulnerability};
use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, result::Error as DieselError, sql_query, sql_types};
use schema::{comments, deals, exercise_bids, exercises};

no_arg_sql_function!(
	random,
	sql_types::Float,
	"Represents the SQL RANDOM() function."
);

#[derive(Queryable, Serialize)]
pub struct Deal {
	pub id: i32,
	pub dealer: Seat,
	pub vulnerable: Vulnerability,
	pub north: Hand,
	pub east: Hand,
	pub south: Hand,
	pub west: Hand,
}

#[derive(Insertable, Serialize)]
#[table_name = "deals"]
pub struct DealInsert {
	dealer: Seat,
	vulnerable: Vulnerability,
	north: Hand,
	east: Hand,
	south: Hand,
	west: Hand,
}

impl DealInsert {
	fn insert(&self, conn: &SqliteConnection) -> Result<Deal, DieselError> {
		use schema::deals::dsl::*;
		insert_into(deals).values(self).execute(conn)?;
		deals.order(id.desc()).first(conn)
	}
}

impl Deal {
	pub fn create_random(conn: &SqliteConnection) -> Result<Deal, DieselError> {
		Self::random().insert(conn)
	}

	pub fn random() -> DealInsert {
		let hands = Deck::deal();
		DealInsert {
			dealer: Seat::North,
			vulnerable: Vulnerability::Neither,
			north: hands.0,
			east: hands.1,
			south: hands.2,
			west: hands.3,
		}
	}

	pub fn by_id(conn: &SqliteConnection, deal_id: i32) -> Result<Deal, DieselError> {
		use schema::deals::dsl::*;
		deals.filter(id.eq(deal_id)).first(conn)
	}
}

#[derive(Queryable, QueryableByName, Clone, Serialize)]
#[table_name = "exercises"]
pub struct Exercise {
	pub id: i32,
	pub deal_id: i32,
	pub bids: BidSequence,
	pub parent_id: Option<i32>,
	pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "exercises"]
struct ExerciseInsert {
	deal_id: i32,
	bids: BidSequence,
	parent_id: Option<i32>,
}

impl ExerciseInsert {
	fn insert(&self, conn: &SqliteConnection) -> Result<Exercise, DieselError> {
		use schema::exercises::dsl::*;
		insert_into(exercises).values(self).execute(conn)?;
		exercises.order(id.desc()).first(conn)
	}
}

impl Exercise {
	pub fn create(conn: &SqliteConnection, deal: &Deal) -> Result<Exercise, DieselError> {
		Self::new(deal.id).insert(conn)
	}

	fn new(deal_id: i32) -> ExerciseInsert {
		ExerciseInsert {
			deal_id,
			bids: BidSequence::empty(),
			parent_id: None,
		}
	}

	pub fn create_comment(
		&self,
		conn: &SqliteConnection,
		user: &User,
		text: String,
	) -> Result<Comment, DieselError> {
		Comment::new(text, user.id, self.id).insert(conn)
	}

	pub fn by_id(conn: &SqliteConnection, ex_id: i32) -> Result<Exercise, DieselError> {
		use schema::exercises::dsl::*;
		exercises.filter(id.eq(ex_id)).first(conn)
	}

	pub fn get_comments(&self, conn: &SqliteConnection) -> Result<Vec<Comment>, DieselError> {
		Comment::by_exercise_id(conn, self.id)
	}

	pub fn get_random(conn: &SqliteConnection) -> Result<Exercise, DieselError> {
		use schema::exercises::dsl::*;
		exercises.order(random).first(conn)
	}

	pub fn get_unbid_or_create(
		conn: &SqliteConnection,
		user: &User,
	) -> Result<Exercise, DieselError> {
		let unbid = Self::get_unbid(conn, user)?;
		if unbid.len() > 0 {
			return Ok(unbid[0].clone());
		}

		let deal = Deal::create_random(&conn).expect("failed to create random deal");
		Self::create(conn, &deal)
	}

	fn get_unbid(conn: &SqliteConnection, user: &User) -> Result<Vec<Exercise>, DieselError> {
		// get deal_id associated with the last n exercises bid by the current user
		let lookback = 5;
		let recent_deal_ids = format!(
			"select ex.deal_id
             from exercise_bids bid join exercises ex on bid.exercise_id = ex.id
             where bid.user_id = {}
             order by bid.created desc
             limit {}",
			user.id, lookback,
		);

		// get user's bid exercise ids
		let user_ex_ids = format!(
			"select exercise_id from exercise_bids where user_id = {}",
			user.id,
		);

		// get unbid exercises whose deals have also not been recently bid
		let query = format!(
            "select * from exercises where deal_id not in ({}) and id not in ({}) order by random()",
            recent_deal_ids, user_ex_ids,
        );
		sql_query(query).load(conn)
	}

	fn new_followup(&self, bid: &Bid) -> Option<ExerciseInsert> {
		let mut new_ex = Self::new(self.deal_id);
		new_ex.parent_id = Some(self.id);
		match self.bids.with_continuation(bid) {
			Ok(bids) => {
				new_ex.bids = bids;
				Some(new_ex)
			}
			Err(_) => None,
		}
	}

	pub fn insert_bid(
		&self,
		conn: &SqliteConnection,
		uid: i32,
		new_bid: &Bid,
	) -> Result<ExerciseBid, DieselError> {
		use schema::exercise_bids::dsl::*;

		insert_into(exercise_bids)
			.values(self.build_bid(uid, &new_bid))
			.execute(conn)?; // failed to insert bid

		exercise_bids.order(id.desc()).first(conn)
	}

	fn build_bid(&self, user_id: i32, bid: &Bid) -> ExerciseBidInsert {
		if !self.bids.valid_continuation(bid) {
			panic!("invalid continuation passed");
		}
		ExerciseBidInsert {
			exercise_id: self.id,
			user_id,
			bid: bid.clone(),
		}
	}
}
#[derive(Queryable, Serialize)]
pub struct ExerciseBid {
	pub id: i32,
	pub created: NaiveDateTime,
	pub exercise_id: i32,
	pub user_id: i32,
	pub bid: Bid,
}

#[derive(Insertable)]
#[table_name = "exercise_bids"]
pub struct ExerciseBidInsert {
	exercise_id: i32,
	user_id: i32,
	bid: Bid,
}

impl ExerciseBid {
	pub fn create_followup_exercise(
		&self,
		conn: &SqliteConnection,
	) -> Result<Option<Exercise>, DieselError> {
		let ex = Exercise::by_id(conn, self.exercise_id)?;
		match ex.new_followup(&self.bid) {
			Some(ref followup) if !followup.bids.is_finished() => followup.insert(conn).map(Some),
			_ => Ok(None),
		}
	}

	pub fn by_id(conn: &SqliteConnection, ex_bid_id: i32) -> Result<ExerciseBid, DieselError> {
		use schema::exercise_bids::dsl::*;
		exercise_bids.filter(id.eq(ex_bid_id)).first(conn)
	}

	pub fn by_exercise_id(
		conn: &SqliteConnection,
		ex_id: i32,
	) -> Result<Vec<ExerciseBid>, DieselError> {
		use schema::exercise_bids::dsl::*;
		exercise_bids.filter(exercise_id.eq(ex_id)).load(conn)
	}
}

#[derive(Queryable, Serialize)]
pub struct Comment {
	pub id: i32,
	pub user_id: i32,
	pub exercise_id: i32,
	pub text: String,
	pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "comments"]
struct CommentInsert {
	user_id: i32,
	exercise_id: i32,
	text: String,
}

impl CommentInsert {
	fn insert(&self, conn: &SqliteConnection) -> Result<Comment, DieselError> {
		use schema::comments::dsl::*;
		insert_into(comments).values(self).execute(conn)?;
		comments.order(id.desc()).first(conn)
	}
}

impl Comment {
	fn new(text: String, user_id: i32, exercise_id: i32) -> CommentInsert {
		CommentInsert {
			text,
			user_id,
			exercise_id,
		}
	}

	fn by_exercise_id(conn: &SqliteConnection, ex_id: i32) -> Result<Vec<Comment>, DieselError> {
		use schema::comments::dsl::*;
		comments.filter(exercise_id.eq(ex_id)).load(conn)
	}
}

pub fn get_random_conflicting_exercise(
	conn: &SqliteConnection,
	user: &User,
) -> Result<Vec<Exercise>, DieselError> {
	let conflicting_exercise_ids = format!(
		"select exercise_id from exercise_bids
         where exercise_id in (select distinct exercise_id from exercise_bids where user_id = {})
         group by exercise_id
         having count(distinct bid) > 1",
		user.id,
	);
	let query = format!(
		"select * from exercises where id in ({}) order by random()",
		conflicting_exercise_ids
	);
	sql_query(query).load(conn)
}
