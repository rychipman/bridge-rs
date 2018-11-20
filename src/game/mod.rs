#[cfg(test)]
mod tests;

use std::fmt;

struct Contract(Level, Trump);

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Card(Rank, Suit);

impl Card {
    fn suit(&self) -> Suit {
        self.1
    }

    fn rank(&self) -> Rank {
        self.0
    }
}

enum Bid {
    Contract(Contract),
    Pass,
    Double,
    Redouble,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

enum Trump {
    NoTrump,
    Trump(Suit),
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Level {
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
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

struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Deck {
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

struct Hand(Vec<Card>);

impl Hand {
    fn random() -> Hand {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut cards = Deck::new().0.into_iter().choose_multiple(&mut rng, 13);
        cards.sort();
        Hand(cards)
    }

    fn suit_holding(&self, suit: Suit) -> Vec<Card> {
        self.0
            .clone()
            .into_iter()
            .filter(|c| c.suit() == suit)
            .collect()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spades: String = self
            .suit_holding(Suit::Spades)
            .iter()
            .map(|c| format!("{}", c.rank()))
            .collect();
        write!(f, "{}", spades)?;

        let hearts: String = self
            .suit_holding(Suit::Hearts)
            .iter()
            .map(|c| format!("{}", c.rank()))
            .collect();
        write!(f, "|{}", hearts)?;

        let diamonds: String = self
            .suit_holding(Suit::Diamonds)
            .iter()
            .map(|c| format!("{}", c.rank()))
            .collect();
        write!(f, "|{}", diamonds)?;

        let clubs: String = self
            .suit_holding(Suit::Clubs)
            .iter()
            .map(|c| format!("{}", c.rank()))
            .collect();
        write!(f, "|{}", clubs)
    }
}
