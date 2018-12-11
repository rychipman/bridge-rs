use super::game::{Bid, BidSequence, Hand, Seat, Suit, Vulnerability};
use diesel::{delete, insert_into, prelude::*};
use std::{fmt, io};

mod schema {
    table! {
        current_user (id) {
            id -> Integer,
            user_id -> Integer,
        }
    }

    table! {
        deals (id) {
            id -> Integer,
            dealer -> Text,
            vulnerable -> Text,
            hand -> Text,
        }
    }

    table! {
        exercise_bids (id) {
            id -> Integer,
            exercise_id -> Integer,
            user_id -> Integer,
            bid -> Text,
        }
    }

    table! {
        exercises (id) {
            id -> Integer,
            deal_id -> Integer,
            bids -> Text,
        }
    }

    table! {
        users (id) {
            id -> Integer,
            email -> Text,
        }
    }

    joinable!(current_user -> users (user_id));
    joinable!(exercise_bids -> exercises (exercise_id));
    joinable!(exercise_bids -> users (user_id));
    joinable!(exercises -> deals (deal_id));

    allow_tables_to_appear_in_same_query!(current_user, deals, exercise_bids, exercises, users,);
}
use self::schema::{deals, exercise_bids, exercises, users};

pub fn connect_db() -> SqliteConnection {
    SqliteConnection::establish("/Users/ryan/git/rust/bridge/bridge.sqlite")
        .expect("failed to connect to db")
}

pub fn current_user() -> Option<User> {
    use self::schema::{current_user::dsl::current_user, users::dsl::*};
    current_user
        .inner_join(users)
        .select((id, email))
        .first::<User>(&connect_db())
        .ok()
}

pub fn get_user(user_email: &str) -> User {
    use self::schema::users::dsl::*;
    users
        .filter(email.eq(user_email))
        .first(&connect_db())
        .expect("failed to find user")
}

pub fn login(user_email: &str) {
    use self::schema::current_user::dsl::*;
    let user = get_user(user_email);
    insert_into(current_user)
        .values(user_id.eq(user.id))
        .execute(&connect_db())
        .expect("failed to log user in");
}

pub fn register(user_email: &str) {
    use self::schema::users::dsl::*;
    insert_into(users)
        .values(User::new(user_email))
        .execute(&connect_db())
        .expect("failed to register user");
}

pub fn logout() {
    use self::schema::current_user::dsl::current_user;
    delete(current_user)
        .execute(&connect_db())
        .expect("failed to log user out");
}

pub fn bid_interactively() {
    // check if we are logged in
    let user = current_user().expect("must be logged in");

    // generate a deal
    let deal = generate_deal();

    // generate an exercise with that deal
    let exercise = generate_exercise(&deal);

    // print the deal and exercise
    println!("{}{}", deal, exercise);

    // prompt the user to bid on it
    println!("Please Enter Your Bid.");
    let mut bid = String::new();
    io::stdin()
        .read_line(&mut bid)
        .expect("failed to read user bid input");

    // parse the user's bid
    let bid = Bid::parse(&bid.trim());

    // turn the user's bid into an exercisebid
    let ex_bid = exercise.insert_bid(user.id, bid);

    // debug printing
    println!("your bid: {:?}", ex_bid);
}

fn generate_deal() -> Deal {
    use self::schema::deals::dsl::*;

    insert_into(deals)
        .values(Deal::random())
        .execute(&connect_db())
        .expect("failed to insert new deal");

    deals
        .order(id.desc())
        .first(&connect_db())
        .expect("failed to retrieve newest deal")
}

fn generate_exercise(deal: &Deal) -> Exercise {
    use self::schema::exercises::dsl::*;

    insert_into(exercises)
        .values(Exercise::new(deal.id))
        .execute(&connect_db())
        .expect("failed to insert new exercise");

    exercises
        .order(id.desc())
        .first(&connect_db())
        .expect("failed to retrieve newest exercise")
}

#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct User {
    id: i32,
    email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct UserInsert {
    email: String,
}

impl User {
    fn new(email: &str) -> UserInsert {
        UserInsert {
            email: email.to_string(),
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Exercise)]
#[belongs_to(User)]
struct ExerciseBid {
    id: i32,
    exercise_id: i32,
    user_id: i32,
    bid: Bid,
}

#[derive(Insertable)]
#[table_name = "exercise_bids"]
struct ExerciseBidInsert {
    exercise_id: i32,
    user_id: i32,
    bid: Bid,
}

impl fmt::Display for ExerciseBid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Next Bid: {}", self.bid)
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Deal)]
struct Exercise {
    id: i32,
    deal_id: i32,
    bids: BidSequence,
}

#[derive(Insertable)]
#[table_name = "exercises"]
struct ExerciseInsert {
    deal_id: i32,
    bids: BidSequence,
}

impl Exercise {
    fn new(deal_id: i32) -> ExerciseInsert {
        ExerciseInsert {
            deal_id,
            bids: BidSequence::empty(),
        }
    }

    fn insert_bid(&self, uid: i32, new_bid: Bid) -> ExerciseBid {
        use self::schema::exercise_bids::dsl::*;

        insert_into(exercise_bids)
            .values(self.build_bid(uid, new_bid))
            .execute(&connect_db())
            .expect("failed to insert bid");

        exercise_bids
            .order(id.desc())
            .first(&connect_db())
            .expect("failed to get newest ExerciseBid")
    }

    fn build_bid(&self, user_id: i32, bid: Bid) -> ExerciseBidInsert {
        if !self.bids.valid_continuation(&bid) {
            panic!("invalid bid")
        }
        ExerciseBidInsert {
            exercise_id: self.id,
            user_id,
            bid,
        }
    }
}

impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.bids.fmt_table(f, Seat::North)
    }
}

#[derive(Queryable, Identifiable)]
pub struct Deal {
    id: i32,
    dealer: Seat,
    vulnerable: Vulnerability,
    hand: Hand,
}

#[derive(Insertable)]
#[table_name = "deals"]
pub struct DealInsert {
    hand: Hand,
    dealer: Seat,
    vulnerable: Vulnerability,
}

impl Deal {
    pub fn random() -> DealInsert {
        DealInsert {
            hand: Hand::random(),
            dealer: Seat::North,
            vulnerable: Vulnerability::Neither,
        }
    }
}

impl fmt::Display for Deal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dealer = format!("{}", self.dealer);
        let vulnerable = format!("{}", self.vulnerable);

        let spades = format!("{}", self.hand.suit_holding(Suit::Spades));
        let hearts = format!("{}", self.hand.suit_holding(Suit::Hearts));
        let diamonds = format!("{}", self.hand.suit_holding(Suit::Diamonds));
        let clubs = format!("{}", self.hand.suit_holding(Suit::Clubs));

        writeln!(f, "+-----------------------+")?;
        writeln!(f, "|     Dealer: {:<10}|", dealer)?;
        writeln!(f, "+-----------------------+")?;
        writeln!(f, "| Vulnerable: {:<10}|", vulnerable)?;
        writeln!(f, "+-----------------------+")?;
        writeln!(f, "|   Spades: {:<12}|", spades)?;
        writeln!(f, "|   Hearts: {:<12}|", hearts)?;
        writeln!(f, "| Diamonds: {:<12}|", diamonds)?;
        writeln!(f, "|    Clubs: {:<12}|", clubs)?;
        writeln!(f, "+-----------------------+")
    }
}
