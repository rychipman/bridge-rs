#[cfg(test)]
mod tests;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};
use std::{fmt, io::Write};

pub struct Contract(Level, Trump);

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

pub enum Vulnerability {
    NS,
    EW,
    Both,
    Neither,
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

pub enum Seat {
    North,
    South,
    East,
    West,
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

pub enum Bid {
    Contract(Contract),
    Pass,
    Double,
    Redouble,
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
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

pub enum Trump {
    NoTrump,
    Trump(Suit),
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Level {
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
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
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
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
        Hand(cards)
    }

    pub fn random() -> Hand {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let cards = Deck::new().0.into_iter().choose_multiple(&mut rng, 13);
        Hand::new(cards)
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
