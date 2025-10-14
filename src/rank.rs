use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Rank {
    One,
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
    Sota,
    Caballo,
    Rey,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
            Self::Sota => "S",
            Self::Caballo => "C",
            Self::Rey => "R",
        };
        write!(f, "{}", s)
    }
}

impl Rank {
    pub const FRENCH_RANKS: [Rank; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
    pub const SPANISH_RANKS: [Rank; 10] = [
        Self::Two,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Sota,
        Self::Caballo,
        Self::Rey,
        Self::Three,
        Self::One,
    ];
}
