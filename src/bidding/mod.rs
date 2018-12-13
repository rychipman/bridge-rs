use super::game::{Bid, BidSequence, Deck, Hand, Seat, Suit, Vulnerability};
use diesel::{delete, insert_into, prelude::*};
use std::{
    fmt::{self, Write},
    io,
};

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
            north -> Text,
            east -> Text,
            south -> Text,
            west -> Text,
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

pub fn bid_opening() {
    // generate a deal
    let deal = generate_deal();

    // generate an exercise with that deal
    let exercise = generate_exercise(&deal);

    // prompt user to bid the exercise
    bid_interactively(&deal, &exercise);
}

pub fn bid_continuation() {
    // find an unbid continuation exercise
    let exercise = find_unbid_continuation();

    // get the exercise's deal
    let deal = Deal::get(exercise.deal_id);

    // prompt user to bid the exercise
    bid_interactively(&deal, &exercise);
}

fn find_unbid_continuation() -> Exercise {
    use self::schema::{
        exercise_bids::dsl::{exercise_bids, exercise_id, user_id},
        exercises::dsl::{exercises, id},
    };
    let user = current_user().expect("must be logged in");
    let subquery = exercise_bids
        .select(exercise_id)
        .filter(user_id.eq(user.id));
    exercises
        .filter(id.ne_all(subquery))
        .first(&connect_db())
        .expect("failed to get an unbid continuation")
}

fn bid_interactively(deal: &Deal, exercise: &Exercise) {
    // check if we are logged in
    let user = current_user().expect("must be logged in");

    // print the deal and exercise
    let next_seat = exercise.bids.next_seat(deal.dealer);
    println!(
        "{}{}{}",
        deal.header(),
        deal.view_for_seat(next_seat),
        exercise
    );

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

    // create follow-up exercise
    let followup_ex = ex_bid.create_followup_exercise().insert();

    // debug printing
    println!("created followup exercise with id {}", followup_ex.id);
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

impl ExerciseBid {
    fn create_followup_exercise(&self) -> ExerciseInsert {
        Exercise::get(self.exercise_id).create_followup(&self.bid)
    }
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

impl ExerciseInsert {
    fn insert(&self) -> Exercise {
        use self::schema::exercises::dsl::*;

        insert_into(exercises)
            .values(self)
            .execute(&connect_db())
            .expect("failed to insert exercise");

        exercises
            .order(id.desc())
            .first(&connect_db())
            .expect("failed to get most recent exercise")
    }
}

impl Exercise {
    fn new(deal_id: i32) -> ExerciseInsert {
        ExerciseInsert {
            deal_id,
            bids: BidSequence::empty(),
        }
    }

    fn get(ex_id: i32) -> Exercise {
        use self::schema::exercises::dsl::*;
        exercises
            .filter(id.eq(ex_id))
            .first(&connect_db())
            .expect("failed to get exercise with id")
    }

    fn create_followup(&self, bid: &Bid) -> ExerciseInsert {
        let mut new_ex = Self::new(self.deal_id);
        new_ex.bids = self.bids.with_continuation(bid);
        new_ex
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
    north: Hand,
    east: Hand,
    south: Hand,
    west: Hand,
}

#[derive(Insertable)]
#[table_name = "deals"]
pub struct DealInsert {
    dealer: Seat,
    vulnerable: Vulnerability,
    north: Hand,
    east: Hand,
    south: Hand,
    west: Hand,
}

impl Deal {
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

    pub fn get(deal_id: i32) -> Deal {
        use self::schema::deals::dsl::*;
        deals
            .filter(id.eq(deal_id))
            .first(&connect_db())
            .expect("failed to find deal by id")
    }

    fn hand_for_seat(&self, seat: Seat) -> &Hand {
        match seat {
            Seat::North => &self.north,
            Seat::East => &self.east,
            Seat::South => &self.south,
            Seat::West => &self.west,
        }
    }

    fn header(&self) -> String {
        let dealer = format!("{}", self.dealer);
        let vulnerable = format!("{}", self.vulnerable);

        let mut out = String::new();
        writeln!(out, "+-----------------------+").unwrap();
        writeln!(out, "|     Dealer: {:<10}|", dealer).unwrap();
        writeln!(out, "+-----------------------+").unwrap();
        writeln!(out, "| Vulnerable: {:<10}|", vulnerable).unwrap();
        writeln!(out, "+-----------------------+").unwrap();

        out
    }

    fn view_for_seat(&self, seat: Seat) -> String {
        let hand = self.hand_for_seat(seat);
        let header = match seat {
            Seat::North => "NORTH",
            Seat::East => " EAST",
            Seat::South => "SOUTH",
            Seat::West => " WEST",
        };

        let spades = format!("{}", hand.suit_holding(Suit::Spades));
        let hearts = format!("{}", hand.suit_holding(Suit::Hearts));
        let diamonds = format!("{}", hand.suit_holding(Suit::Diamonds));
        let clubs = format!("{}", hand.suit_holding(Suit::Clubs));

        let mut out = String::new();
        writeln!(&mut out, "|          {}        |", header).unwrap();
        writeln!(&mut out, "|   Spades: {:<12}|", spades).unwrap();
        writeln!(&mut out, "|   Hearts: {:<12}|", hearts).unwrap();
        writeln!(&mut out, "| Diamonds: {:<12}|", diamonds).unwrap();
        writeln!(&mut out, "|    Clubs: {:<12}|", clubs).unwrap();
        writeln!(&mut out, "+-----------------------+").unwrap();

        out
    }
}

impl fmt::Display for Deal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.header())?;
        write!(f, "{}", self.view_for_seat(Seat::North))?;
        write!(f, "{}", self.view_for_seat(Seat::East))?;
        write!(f, "{}", self.view_for_seat(Seat::South))?;
        write!(f, "{}", self.view_for_seat(Seat::West))
    }
}
