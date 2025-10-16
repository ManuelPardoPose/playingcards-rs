use std::fmt::{self, Display, Formatter};

use crate::{rank::Rank, suit::Suit};

const BG: &str = " ";
const VERT: char = '│';
const HORI: &str = "─";
const TL: char = '╭';
const TR: char = '╮';
const BL: char = '╰';
const BR: char = '╯';
const PADDING: usize = 1;

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

    /// Renders a multiline utf8 representation of the playing card.
    /// (Works fine but was written very sloppily)
    ///
    /// `Panics` if `height` is < 5.
    pub fn render_utf8(&self, height: usize) -> Vec<String> {
        assert!(
            height >= 5,
            "'height' is expected to be >= 5. Otherwise the full card information can not be printed."
        );
        let height = height + height % 2 - 1; // make uneven
        let mut width = height * 4 / 3;
        width = width + width % 2 - 1; // make uneven
        let rank_str = &format!("{}", self.rank);
        let rank_str_len = rank_str.chars().count();
        let suit_str = &format!("{}", self.suit);
        let suit_str_len = suit_str.chars().count();

        let mut lines: Vec<String> = vec![];
        for y in 0..height {
            if y == 0 || y == height - 1 {
                // first and last row
                lines.push(HORI.repeat(width - 2));
                continue;
            }

            let mut line = String::new();
            line.push(VERT); // left border

            if y == PADDING || y == PADDING + 1 {
                // top left
                line.push_str(&BG.repeat(PADDING));
                if y == PADDING {
                    line.push_str(rank_str);
                    line.push_str(&BG.repeat(width - PADDING - rank_str_len - 2));
                } else {
                    line.push_str(suit_str);
                    line.push_str(&BG.repeat(width - PADDING - suit_str_len - 2));
                }
            } else if y == height - 1 - PADDING || y == height - 2 - PADDING {
                // bottom right
                if y == height - 1 - PADDING {
                    line.push_str(&BG.repeat(width - PADDING - rank_str_len - 2));
                    line.push_str(rank_str);
                } else {
                    line.push_str(&BG.repeat(width - PADDING - suit_str_len - 2));
                    line.push_str(suit_str);
                }
                line.push_str(&BG.repeat(PADDING));
            } else if y == height / 2 {
                // center
                let offset = (width - 2) / 2;
                line.push_str(&BG.repeat(offset));
                line.push_str(suit_str);
                line.push_str(&BG.repeat(width - 2 - offset - suit_str_len));
            } else {
                line.push_str(&BG.repeat(width - 2));
            }

            line.push(VERT); // right border
            lines.push(line);
        }
        // corners
        lines[0].insert(0, TL);
        lines[0].push(TR);
        lines[height - 1].insert(0, BL);
        lines[height - 1].push(BR);
        lines
    }
}
