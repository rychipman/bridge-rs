use super::game::{Bid, BidSequence, Hand, Seat, Suit, Vulnerability};
use diesel::{delete, insert_into, prelude::*};
use std::fmt;

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

pub fn logout() {
    use self::schema::current_user::dsl::current_user;
    delete(current_user)
        .execute(&connect_db())
        .expect("failed to log user out");
}

pub fn find_or_create_user(user_email: String) {
    use self::schema::users::dsl::*;
    let users_with_email = users
        .filter(email.eq(&user_email))
        .load::<User>(&connect_db())
        .expect("failed to query users table");
    match users_with_email.len() {
        0 => {
            println!("no users with provided email...creating");
            create_user(user_email)
        }
        1 => println!("found user: {:?}", users_with_email[0]),
        _ => println!("found more than 1 user with email"),
    }
}

fn create_user(user_email: String) {
    use self::schema::users::dsl::*;
    insert_into(users)
        .values(User::new(user_email))
        .execute(&connect_db())
        .expect("failed to insert user");
}

pub fn play_arbitrary_exercise() {
    use self::schema::exercises::dsl::exercises;
    let ex = exercises
        .first::<Exercise>(&connect_db())
        .expect("failed to get an exercise");
    ex.insert_bid(Bid::parse("1S"));
}

pub fn generate_exercise() {
    use self::schema::exercises::dsl::*;
    let deal = get_deal();
    let ex = Exercise::new(deal.id);
    insert_into(exercises)
        .values(ex)
        .execute(&connect_db())
        .unwrap();
}

pub fn show_exercises_with_bids() {
    use self::schema::{deals::dsl::deals, exercise_bids::dsl::exercise_bids, exercises::dsl::*};
    let res = exercises
        .inner_join(deals)
        .inner_join(exercise_bids)
        .load::<(Exercise, Deal, ExerciseBid)>(&connect_db())
        .expect("error loading exercises");
    for (ex, deal, ex_bid) in res {
        println!("{}{}{}", deal, ex, ex_bid);
    }
}

pub fn get_deal() -> Deal {
    use self::schema::deals::dsl::*;
    deals
        .first::<Deal>(&connect_db())
        .expect("error loading deal")
}

pub fn generate_deals(n: usize) {
    use self::schema::deals::dsl::*;

    let conn = connect_db();
    for _ in 0..n {
        let _count = insert_into(deals)
            .values(Deal::random())
            .execute(&conn)
            .unwrap();
    }
    println!("generated {} deal(s)", n);
}

pub fn show_deals() {
    use self::schema::deals::dsl::*;

    let dls = deals
        .load::<Deal>(&connect_db())
        .expect("error loading deals");

    println!("deals in db:");
    for deal in dls {
        println!("{}", deal);
    }
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
    fn new(email: String) -> UserInsert {
        UserInsert { email }
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Exercise)]
struct ExerciseBid {
    id: i32,
    exercise_id: i32,
    bid: Bid,
}

#[derive(Insertable)]
#[table_name = "exercise_bids"]
struct ExerciseBidInsert {
    exercise_id: i32,
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

    fn insert_bid(&self, new_bid: Bid) {
        use self::schema::exercise_bids::dsl::*;
        insert_into(exercise_bids)
            .values(self.build_bid(new_bid))
            .execute(&connect_db())
            .expect("failed to insert bid");
    }

    fn build_bid(&self, bid: Bid) -> ExerciseBidInsert {
        ExerciseBidInsert {
            exercise_id: self.id,
            bid: bid,
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
