use std::fmt::{self, Display, Formatter};

use rand::seq::SliceRandom;

use crate::{card::Card, rank::Rank, suit::Suit};

pub enum DeckType {
    FRENCH,
    SPANISH,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Deck {
    pub cards: Vec<Card>,
}

pub type Hand = Deck;

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut cards = self.cards.iter();
        if let Some(card) = cards.next() {
            write!(f, "{}", card)?;
        }
        for card in cards {
            write!(f, ", {}", card)?;
        }
        Ok(())
    }
}

impl From<Vec<Card>> for Deck {
    fn from(value: Vec<Card>) -> Self {
        Self { cards: value }
    }
}

impl Deck {
    pub fn new(deck_type: DeckType) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        match deck_type {
            DeckType::FRENCH => {
                for suit in Suit::FRENCH_SUITS {
                    for rank in Rank::FRENCH_RANKS {
                        cards.push(Card::new(rank, suit));
                    }
                }
            }
            DeckType::SPANISH => {
                for suit in Suit::SPANISH_SUITS {
                    for rank in Rank::SPANISH_RANKS {
                        cards.push(Card::new(rank, suit));
                    }
                }
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{deck::Deck, suit::Suit};

    use super::*;

    #[test]
    fn deck_creation() {
        let french_deck = Deck::new(DeckType::FRENCH);
        assert_eq!(french_deck.len(), 52);
        let spanish_deck = Deck::new(DeckType::SPANISH);
        assert_eq!(spanish_deck.len(), 40);
    }

    #[test]
    fn test_eq() {
        let deck = Deck::new(DeckType::FRENCH);
        let deck_copy = deck.clone();
        assert_eq!(deck, deck_copy);
    }

    #[test]
    fn test_shuffle() {
        let deck = Deck::new(DeckType::FRENCH);
        let mut spanish_deck_copy = deck.clone();
        spanish_deck_copy.shuffle();
        assert_ne!(deck, spanish_deck_copy);
    }

    #[test]
    fn test_pop_peek() {
        let mut deck = Deck::new(DeckType::FRENCH);
        assert!(deck.peek().is_some());
        let first_peek = *deck.peek().unwrap();
        assert_eq!(first_peek, Card::new(Rank::Ace, Suit::FrenchSpades));
        let second_peek = *deck.peek().unwrap();
        assert_eq!(first_peek, second_peek);

        let first_pop = {
            let card = deck.pop();
            assert!(card.is_some());
            card.unwrap()
        };
        assert_eq!(first_peek, first_pop);
        let second_pop = {
            let card = deck.pop();
            assert!(card.is_some());
            card.unwrap()
        };
        assert_ne!(first_pop, second_pop);
    }

    #[test]
    fn test_pop_all_cards() {
        let mut deck = Deck::new(DeckType::FRENCH);
        let mut deck2 = Deck::new(DeckType::FRENCH);
        for _ in 0..deck.len() {
            assert!(deck.peek().is_some());
            assert!(deck2.peek().is_some());
            assert_eq!(deck.pop(), deck2.pop());
        }
        assert!(deck.pop().is_none());
        assert!(deck2.pop().is_none());
        assert!(deck.is_empty());
        assert!(deck2.is_empty());
    }

    #[test]
    fn test_from_card_vec() {
        let deck = Deck::new(DeckType::FRENCH);
        let deck2 = Deck::from(deck.cards.clone());
        assert_eq!(deck, deck2);
    }

    #[test]
    fn test_push_card() {
        let mut deck = Deck::new(DeckType::FRENCH);
        assert_eq!(deck.len(), 52);
        let card = Card::new(Rank::Rey, Suit::SpanishCups);
        deck.push(card.clone());
        assert_eq!(deck.len(), 53);
        assert_eq!(deck.pop().unwrap(), card);
    }
}
