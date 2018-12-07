use super::game::{Bid, Hand, Seat, Suit, Vulnerability};
use diesel::{insert_into, prelude::*};
use std::fmt;

mod schema {
    table! {
        hands (id) {
            id -> Integer,
            cards -> Text,
        }
    }
}

pub fn connect_db() -> SqliteConnection {
    SqliteConnection::establish("/Users/ryan/git/rust/bridge/bridge.sqlite")
        .expect("failed to connect to db")
}

pub fn generate_deals(n: usize) {
    use self::schema::hands::dsl::*;

    let conn = connect_db();
    for _ in 0..n {
        let _count = insert_into(hands)
            .values(cards.eq(Hand::random()))
            .execute(&conn)
            .unwrap();
    }
    println!("generated {} hand(s)", n);
}

pub fn show_deals() {
    use self::schema::hands::dsl::*;

    let h = hands
        .select(cards)
        .load::<Hand>(&connect_db())
        .expect("error loading hands");

    println!("hands in db:");
    for hand in h {
        println!("    {}", hand);
    }
}

struct Exercise {
    bids: Vec<Bid>,
    next_bid: Option<Bid>,
}

pub struct Deal {
    hand: Hand,
    dealer: Seat,
    vulnerable: Vulnerability,
}

impl Deal {
    pub fn random() -> Deal {
        Deal {
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
