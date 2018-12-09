use super::game::{Bid, BidSequence, Hand, Seat, Suit, Vulnerability};
use diesel::{insert_into, prelude::*};
use std::fmt;

mod schema {
    table! {
        deals (id) {
            id -> Integer,
            dealer -> Text,
            vulnerable -> Text,
            hand -> Text,
        }
    }

    table! {
        exercises (id) {
            id -> Integer,
            deal_id -> Integer,
            bids -> Text,
            next_bid -> Nullable<Text>,
        }
    }

    joinable!(exercises -> deals (deal_id));

    allow_tables_to_appear_in_same_query!(deals, exercises,);
}
use self::schema::{deals, exercises};

pub fn connect_db() -> SqliteConnection {
    SqliteConnection::establish("/Users/ryan/git/rust/bridge/bridge.sqlite")
        .expect("failed to connect to db")
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

pub fn show_exercises() {
    use self::schema::{deals::dsl::deals, exercises::dsl::*};
    let res = exercises
        .inner_join(deals)
        .load::<(Exercise, Deal)>(&connect_db())
        .expect("error loading exercises");
    for (ex, deal) in res {
        println!("{}{}", deal, ex);
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

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Deal)]
struct Exercise {
    id: i32,
    deal_id: i32,
    bids: BidSequence,
    next_bid: Option<Bid>,
}

#[derive(Insertable)]
#[table_name = "exercises"]
struct ExerciseInsert {
    deal_id: i32,
    bids: BidSequence,
    next_bid: Option<Bid>,
}

impl Exercise {
    fn new(deal_id: i32) -> ExerciseInsert {
        ExerciseInsert {
            deal_id,
            bids: BidSequence::empty(),
            next_bid: None,
        }
    }
}

impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let next_bid = if let Some(ref bid) = self.next_bid {
            format!("{}", bid)
        } else {
            "?".to_string()
        };

        self.bids.fmt_table(f, Seat::North)?;
        writeln!(f, "Next Bid: {}", next_bid)
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
