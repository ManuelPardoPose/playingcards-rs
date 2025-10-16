use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    FrenchHearts,
    FrenchDiamonds,
    FrenchClubs,
    FrenchSpades,
    SpanishCups,
    SpanishCoins,
    SpanishClubs,
    SpanishSwords,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::FrenchHearts => '♥',
            Self::FrenchDiamonds => '♦',
            Self::FrenchClubs => '♣',
            Self::FrenchSpades => '♠',
            Self::SpanishCups => '⚱',
            Self::SpanishCoins => '⬤',
            Self::SpanishClubs => '⚚',
            Self::SpanishSwords => '⚔',
        };
        write!(f, "{}", s)
    }
}

impl Suit {
    pub const FRENCH_SUITS: [Suit; 4] = [
        Self::FrenchHearts,
        Self::FrenchDiamonds,
        Self::FrenchClubs,
        Self::FrenchSpades,
    ];
    pub const SPANISH_SUITS: [Suit; 4] = [
        Self::SpanishCups,
        Self::SpanishCoins,
        Self::SpanishClubs,
        Self::SpanishSwords,
    ];
}
