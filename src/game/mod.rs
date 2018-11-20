#[cfg(test)]
mod tests;

use std::fmt;

pub struct Contract(Level, Trump);

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

pub enum Trump {
    NoTrump,
    Trump(Suit),
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

impl fmt::Display for SuitCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().map(|c| format!("{}", c.rank())).collect();
        write!(f, "{}", s)
    }
}

pub struct Hand(Vec<Card>);

impl Hand {
    pub fn random() -> Hand {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut cards = Deck::new().0.into_iter().choose_multiple(&mut rng, 13);
        cards.sort();
        Hand(cards)
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
