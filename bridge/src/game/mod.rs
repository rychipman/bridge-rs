#[cfg(test)]
mod tests;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use failure::Error;
use std::{fmt, io::Write};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Contract(Level, Trump);

impl Contract {
    pub fn parse(s: &str) -> Result<Self> {
        if s.len() < 2 {
            return Err(format_err!(
                "contract string must have length of at least 2"
            ));
        }
        let level = Level::parse(&s[0..1])?;
        let trump = Trump::parse(&s[1..])?;
        Ok(Contract(level, trump))
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Serialize)]
#[sql_type = "Text"]
pub enum Vulnerability {
    NS,
    EW,
    Both,
    Neither,
}

impl Vulnerability {
    fn parse(s: &str) -> Result<Vulnerability> {
        use self::Vulnerability::*;
        match s {
            "NS" => Ok(NS),
            "EW" => Ok(EW),
            "Both" => Ok(Both),
            "None" => Ok(Neither),
            _ => Err(format_err!("invalid vulnerability string '{}'", s)),
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
        let vulnerability = Vulnerability::parse(&s).map_err(|e| e.compat())?;
        Ok(vulnerability)
    }
}

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Serialize)]
#[sql_type = "Text"]
pub enum Seat {
    North,
    East,
    South,
    West,
}

impl Seat {
    fn vec() -> Vec<Seat> {
        vec![Seat::North, Seat::East, Seat::South, Seat::West]
    }

    fn parse(s: &str) -> Result<Seat> {
        use self::Seat::*;
        match s {
            "North" => Ok(North),
            "South" => Ok(South),
            "East" => Ok(East),
            "West" => Ok(West),
            _ => Err(format_err!("invalid seat string '{}'", s)),
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
        let seat = Seat::parse(&s).map_err(|e| e.compat())?;
        Ok(seat)
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, Serialize)]
#[sql_type = "Text"]
pub struct BidSequence(Vec<Bid>);

impl BidSequence {
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn new(bids: Vec<Bid>) -> Self {
        BidSequence(bids)
    }

    pub fn bids(&self) -> &[Bid] {
        &self.0
    }

    pub fn next_seat(&self, dealer: Seat) -> Seat {
        let offset = match dealer {
            Seat::North => 0,
            Seat::East => 1,
            Seat::South => 2,
            Seat::West => 3,
        };
        Seat::vec()
            .into_iter()
            .cycle()
            .nth(self.0.len() + offset)
            .unwrap()
    }

    fn parse(s: &str) -> Result<Self> {
        if s.len() == 0 {
            return Ok(Self::empty());
        }
        let bids: Result<Vec<Bid>> = s.split(",").map(Bid::parse).collect();
        match bids {
            Ok(vec) => Ok(BidSequence(vec)),
            Err(e) => Err(e),
        }
    }

    pub fn is_finished(&self) -> bool {
        let len = self.0.len();
        if len < 4 {
            false
        } else if self.0[0] == Bid::Pass
            && self.0[1] == Bid::Pass
            && self.0[2] == Bid::Pass
            && self.0[3] == Bid::Pass
        {
            true
        } else {
            self.0[len - 1] == Bid::Pass
                && self.0[len - 2] == Bid::Pass
                && self.0[len - 3] == Bid::Pass
        }
    }

    fn last_non_pass_bid(&self) -> Option<(usize, &Bid)> {
        for (i, bid) in self.0.iter().enumerate().rev() {
            match bid {
                Bid::Pass => continue,
                _ => return Some((i, bid)),
            }
        }
        None
    }

    fn last_contract_bid(&self) -> Option<(usize, &Bid)> {
        for (i, bid) in self.0.iter().enumerate().rev() {
            match bid {
                Bid::Contract(_) => return Some((i, bid)),
                _ => continue,
            }
        }
        None
    }

    pub fn valid_continuation(&self, next: &Bid) -> bool {
        let lnp = self.last_non_pass_bid();
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
                Some((_, Bid::Contract(c))) => next > &Bid::Contract(c.clone()),
                Some((_, _)) => next > self.last_contract_bid().unwrap().1,
                None => true,
            }
        }
    }

    pub fn with_continuation(&self, next: &Bid) -> Result<BidSequence> {
        if self.valid_continuation(next) {
            let mut new_seq = self.clone();
            new_seq.0.push(next.clone());
            Ok(new_seq)
        } else {
            Err(format_err!("invalid continuation of bid sequence"))
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
        let bids = BidSequence::parse(&s).map_err(|e| e.compat())?;
        Ok(bids)
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, PartialOrd, Serialize)]
#[sql_type = "Text"]
pub enum Bid {
    Pass,
    Double,
    Redouble,
    Contract(Contract),
}

impl Bid {
    pub fn parse(s: &str) -> Result<Self> {
        let bid = match s {
            "Pass" | "P" => Bid::Pass,
            "Dbl" => Bid::Double,
            "Rdbl" => Bid::Redouble,
            _ => Bid::Contract(Contract::parse(s)?),
        };
        Ok(bid)
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
        let bid = Bid::parse(&s).map_err(|e| e.compat())?;
        Ok(bid)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn parse(s: &str) -> Result<Self> {
        use self::Suit::*;
        match s {
            "S" => Ok(Spades),
            "H" => Ok(Hearts),
            "D" => Ok(Diamonds),
            "C" => Ok(Clubs),
            _ => Err(format_err!("failed to parse suit string '{}'", s)),
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum Trump {
    Trump(Suit),
    NoTrump,
}

impl Trump {
    pub fn parse(s: &str) -> Result<Trump> {
        use self::Trump::*;
        match s {
            "NT" => Ok(NoTrump),
            _ => Ok(Trump(Suit::parse(s)?)),
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize)]
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
    pub fn parse(s: &str) -> Result<Level> {
        use self::Level::*;
        match s {
            "7" => Ok(Seven),
            "6" => Ok(Six),
            "5" => Ok(Five),
            "4" => Ok(Four),
            "3" => Ok(Three),
            "2" => Ok(Two),
            "1" => Ok(One),
            _ => Err(format_err!("failed to parse level string '{}'", s)),
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize)]
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
    pub fn parse(s: &str) -> Result<Rank> {
        use self::Rank::*;
        match s {
            "A" => Ok(Ace),
            "K" => Ok(King),
            "Q" => Ok(Queen),
            "J" => Ok(Jack),
            "T" => Ok(Ten),
            "9" => Ok(Nine),
            "8" => Ok(Eight),
            "7" => Ok(Seven),
            "6" => Ok(Six),
            "5" => Ok(Five),
            "4" => Ok(Four),
            "3" => Ok(Three),
            "2" => Ok(Two),
            _ => Err(format_err!("invalid rank string '{}'", s)),
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
                cards.push(Card { rank, suit: *suit })
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

    fn parse(suit: Suit, ranks: &str) -> Result<SuitCards> {
        if ranks.len() == 0 {
            return Ok(SuitCards::empty());
        }
        let cards: Result<Vec<Card>> = ranks
            .chars()
            .map(|c| c.to_string())
            .map(|rank_string| match Rank::parse(&rank_string) {
                Ok(rank) => Ok(Card { rank, suit }),
                Err(e) => Err(e),
            })
            .collect();
        Ok(Self::new(cards?))
    }
}

impl fmt::Display for SuitCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().map(|c| format!("{}", c.rank)).collect();
        write!(f, "{}", s)
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[sql_type = "Text"]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let mut cards = cards.clone();
        cards.sort();
        cards.reverse();
        Hand(cards)
    }

    pub fn parse(s: &str) -> Result<Hand> {
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let suit_cards: Result<Vec<Vec<Card>>> = suits
            .iter()
            .zip(s.split("|"))
            .map(|(suit, ranks)| match SuitCards::parse(*suit, ranks) {
                Ok(suit_cards) => Ok(suit_cards.0),
                Err(e) => Err(e),
            })
            .collect();
        let cards: Vec<Card> = suit_cards?.into_iter().flatten().collect();
        Ok(Hand::new(cards))
    }

    pub fn suit_holding(&self, suit: Suit) -> SuitCards {
        let cards = self
            .0
            .clone()
            .into_iter()
            .filter(|c| c.suit == suit)
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
        let hand = Hand::parse(&s).map_err(|e| e.compat())?;
        Ok(hand)
    }
}
