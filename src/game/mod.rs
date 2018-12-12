#[cfg(test)]
mod tests;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use std::{fmt, io::Write};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Contract(Level, Trump);

impl Contract {
    pub fn parse(s: &str) -> Self {
        if s.len() < 2 {
            panic!("length of contract str must be at least 2");
        }
        let level = Level::parse(&s[0..1]);
        let trump = Trump::parse(&s[1..]);
        Contract(level, trump)
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Card(Rank, Suit);

impl Card {
    pub fn suit(&self) -> Suit {
        self.1
    }

    pub fn rank(&self) -> Rank {
        self.0
    }
}

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Vulnerability {
    NS,
    EW,
    Both,
    Neither,
}

impl Vulnerability {
    fn parse(s: &str) -> Vulnerability {
        use self::Vulnerability::*;
        match s {
            "NS" => NS,
            "EW" => EW,
            "Both" => Both,
            "None" => Neither,
            _ => panic!("invalid vulnerability string '{}'", s),
        }
    }
}

impl fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Vulnerability::*;
        let s = match self {
            NS => "NS",
            EW => "EW",
            Both => "Both",
            Neither => "None",
        };
        write!(f, "{}", s)
    }
}

impl<DB> ToSql<Text, DB> for Vulnerability
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        out.write_fmt(format_args!("{}", self))?;
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Vulnerability
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        let vulnerability = Vulnerability::parse(&s);
        Ok(vulnerability)
    }
}

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum Seat {
    North,
    South,
    East,
    West,
}

impl Seat {
    fn parse(s: &str) -> Seat {
        use self::Seat::*;
        match s {
            "North" => North,
            "South" => South,
            "East" => East,
            "West" => West,
            _ => panic!("invalid seat string '{}'", s),
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Seat::*;
        let s = match self {
            North => "North",
            South => "South",
            East => "East",
            West => "West",
        };
        write!(f, "{}", s)
    }
}

impl<DB> ToSql<Text, DB> for Seat
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        out.write_fmt(format_args!("{}", self))?;
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Seat
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        let seat = Seat::parse(&s);
        Ok(seat)
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct BidSequence(Vec<Bid>);

impl BidSequence {
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    fn new(bids: Vec<Bid>) -> Self {
        BidSequence(bids)
    }

    fn parse(s: &str) -> Self {
        if s.len() == 0 {
            return Self::empty();
        }
        let bids = s.split(",").map(Bid::parse).collect();
        BidSequence(bids)
    }

    fn is_finished(&self) -> bool {
        self.0.len() >= 3
            && self.0[0] == Bid::Pass
            && self.0[1] == Bid::Pass
            && self.0[2] == Bid::Pass
    }

    fn last_non_pass(&self) -> Option<(usize, &Bid)> {
        for (i, bid) in self.0.iter().enumerate().rev() {
            if bid == &Bid::Pass {
                return Some((i, bid));
            }
        }
        None
    }

    pub fn valid_continuation(&self, next: &Bid) -> bool {
        let lnp = self.last_non_pass();
        let curr_idx = self.0.len();
        if self.is_finished() {
            false
        } else if next == &Bid::Pass {
            true
        } else if next == &Bid::Redouble {
            match lnp {
                Some((idx, prev)) => match prev {
                    Bid::Double if (curr_idx - idx) % 2 == 1 => true,
                    _ => false,
                },
                None => false,
            }
        } else if next == &Bid::Double {
            match lnp {
                Some((idx, prev)) => match prev {
                    Bid::Contract(_) if (curr_idx - idx) % 2 == 1 => true,
                    _ => false,
                },
                None => false,
            }
        } else {
            match lnp {
                Some((_, prev)) => next > prev,
                None => true,
            }
        }
    }

    pub fn with_continuation(&self, next: &Bid) -> BidSequence {
        if self.valid_continuation(next) {
            let mut new_seq = self.clone();
            new_seq.0.push(next.clone());
            new_seq
        } else {
            panic!("invalid continuation of bid sequence")
        }
    }

    fn pad_for_table(&self, dealer: Seat) -> Vec<Option<Bid>> {
        // North is first seat shown on table, so we need to add some empty bids
        // if North is not the dealer
        let num_empty = match dealer {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        };
        let mut padded = Vec::new();
        for _ in 0..num_empty {
            padded.push(None)
        }
        for bid in &self.0 {
            padded.push(Some(bid.clone()))
        }
        while padded.len() % 4 != 0 || padded.len() == 0 {
            padded.push(None)
        }
        padded
    }

    pub fn fmt_table(&self, f: &mut fmt::Formatter, dealer: Seat) -> fmt::Result {
        // get string representations of all the bids (padded with empty strings
        // for proper table pagination)
        let bid_strings: Vec<String> = self
            .pad_for_table(dealer)
            .iter()
            .map(|bid_opt| {
                bid_opt
                    .clone()
                    .map_or("".to_string(), |bid| format!("{}", bid))
            })
            .collect();

        // print table header
        writeln!(f, "+-----+-----+-----+-----+")?;
        writeln!(f, "|  N  |  E  |  S  |  W  |")?;
        writeln!(f, "+-----+-----+-----+-----+")?;

        // print rows
        for row in bid_strings.chunks(4) {
            writeln!(
                f,
                "| {:<4}| {:<4}| {:<4}| {:<4}|",
                row[0], row[1], row[2], row[3]
            )?;
            writeln!(f, "+-----+-----+-----+-----+")?;
        }
        Ok(())
    }
}

impl fmt::Display for BidSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|b| format!("{}", b))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", s)
    }
}

impl<DB> ToSql<Text, DB> for BidSequence
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        out.write_fmt(format_args!("{}", self))?;
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for BidSequence
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        let bids = BidSequence::parse(&s);
        Ok(bids)
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, PartialOrd)]
#[sql_type = "Text"]
pub enum Bid {
    Pass,
    Double,
    Redouble,
    Contract(Contract),
}

impl Bid {
    pub fn parse(s: &str) -> Self {
        match s {
            "Pass" => Bid::Pass,
            "Dbl" => Bid::Double,
            "Rdbl" => Bid::Redouble,
            _ => Bid::Contract(Contract::parse(s)),
        }
    }
}

impl fmt::Display for Bid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Bid::*;
        let s = match self {
            Contract(ct) => format!("{}", ct),
            Pass => "Pass".to_string(),
            Double => "Dbl".to_string(),
            Redouble => "Rdbl".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl<DB> ToSql<Text, DB> for Bid
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        out.write_fmt(format_args!("{}", self))?;
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Bid
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        let bid = Bid::parse(&s);
        Ok(bid)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn parse(s: &str) -> Self {
        use self::Suit::*;
        match s {
            "S" => Spades,
            "H" => Hearts,
            "D" => Diamonds,
            "C" => Clubs,
            _ => panic!("invalid suit string '{}'", s),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Suit::*;
        let s = match self {
            Spades => "S",
            Hearts => "H",
            Diamonds => "D",
            Clubs => "C",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Trump {
    Trump(Suit),
    NoTrump,
}

impl Trump {
    pub fn parse(s: &str) -> Trump {
        use self::Trump::*;
        match s {
            "NT" => NoTrump,
            _ => Trump(Suit::parse(s)),
        }
    }
}

impl fmt::Display for Trump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Trump::*;
        let s = match self {
            NoTrump => "NT".to_string(),
            Trump(suit) => format!("{}", suit),
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Level {
    pub fn parse(s: &str) -> Level {
        use self::Level::*;
        match s {
            "7" => Seven,
            "6" => Six,
            "5" => Five,
            "4" => Four,
            "3" => Three,
            "2" => Two,
            "1" => One,
            _ => panic!("invalid level string '{}'", s),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Level::*;
        let s = match self {
            Seven => "7",
            Six => "6",
            Five => "5",
            Four => "4",
            Three => "3",
            Two => "2",
            One => "1",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn parse(s: &str) -> Rank {
        use self::Rank::*;
        match s {
            "A" => Ace,
            "K" => King,
            "Q" => Queen,
            "J" => Jack,
            "T" => Ten,
            "9" => Nine,
            "8" => Eight,
            "7" => Seven,
            "6" => Six,
            "5" => Five,
            "4" => Four,
            "3" => Three,
            "2" => Two,
            _ => panic!("invalid rank string '{}'", s),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Rank::*;
        let s = match self {
            Ace => "A",
            King => "K",
            Queen => "Q",
            Jack => "J",
            Ten => "T",
            Nine => "9",
            Eight => "8",
            Seven => "7",
            Six => "6",
            Five => "5",
            Four => "4",
            Three => "3",
            Two => "2",
        };
        write!(f, "{}", s)
    }
}

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let ranks = vec![
            Rank::Ace,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
            Rank::Two,
        ];
        let suits = vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

        let mut cards = Vec::new();
        for rank in ranks {
            for suit in &suits {
                cards.push(Card(rank, *suit))
            }
        }

        Deck(cards)
    }

    fn shuffled() -> Deck {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut deck = Deck::new();
        deck.0.shuffle(&mut rng);
        deck
    }

    pub fn deal() -> (Hand, Hand, Hand, Hand) {
        let cards = Deck::shuffled().0;
        (
            Hand::new(cards[0..13].to_owned()),
            Hand::new(cards[14..26].to_owned()),
            Hand::new(cards[27..39].to_owned()),
            Hand::new(cards[40..52].to_owned()),
        )
    }
}

pub struct SuitCards(Vec<Card>);

impl SuitCards {
    fn empty() -> SuitCards {
        Self::new(Vec::new())
    }

    fn new(cards: Vec<Card>) -> SuitCards {
        SuitCards(cards)
    }

    fn parse(suit: Suit, ranks: &str) -> SuitCards {
        if ranks.len() == 0 {
            return SuitCards::empty();
        }
        let cards = ranks
            .chars()
            .map(|c| c.to_string())
            .map(|rank_string| {
                let rank = Rank::parse(&rank_string);
                Card(rank, suit)
            })
            .collect();
        Self::new(cards)
    }
}

impl fmt::Display for SuitCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().map(|c| format!("{}", c.rank())).collect();
        write!(f, "{}", s)
    }
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let mut cards = cards.clone();
        cards.sort();
        cards.reverse();
        Hand(cards)
    }

    pub fn parse(s: &str) -> Hand {
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let cards: Vec<Card> = suits
            .iter()
            .zip(s.split("|"))
            .flat_map(|(suit, ranks)| SuitCards::parse(*suit, ranks).0.into_iter())
            .collect();
        Hand::new(cards)
    }

    pub fn suit_holding(&self, suit: Suit) -> SuitCards {
        let cards = self
            .0
            .clone()
            .into_iter()
            .filter(|c| c.suit() == suit)
            .collect();
        SuitCards(cards)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.suit_holding(Suit::Spades))?;
        write!(f, "|{}", self.suit_holding(Suit::Hearts))?;
        write!(f, "|{}", self.suit_holding(Suit::Diamonds))?;
        write!(f, "|{}", self.suit_holding(Suit::Clubs))
    }
}

impl<DB> ToSql<Text, DB> for Hand
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        out.write_fmt(format_args!("{}", self))?;
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Hand
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        let hand = Hand::parse(&s);
        Ok(hand)
    }
}
