use super::game::{Bid, Hand};
use diesel::{insert_into, prelude::*};

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
