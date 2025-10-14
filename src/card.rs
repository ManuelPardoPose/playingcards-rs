use std::fmt::{self, Display, Formatter};

use crate::{rank::Rank, suit::Suit};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{} {}]", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}
