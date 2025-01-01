use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a new SHUFFLED deck
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::NormalCard(NormalCard {
                    suit: suit.clone(),
                    rank,
                }));
            }
        }

        for _ in 0..4 {
            cards.push(Card::SpecialCard(SpecialCard::Wizard));
            cards.push(Card::SpecialCard(SpecialCard::Jester));
        }

        let mut deck = Deck { cards };

        deck.shuffle();

        deck
    }

    /// Shuffle the remaining cards in the deck
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deal N cards
    pub fn deal(&mut self, n: usize) -> Result<Vec<Card>> {
        if n > self.cards.len() {
            bail!(
                "Not enough cards left in deck. Cards remaining: {}; Cards requested: {}",
                self.cards.len(),
                n
            );
        }
        let drain_start = self.cards.len().saturating_sub(n);
        Ok(self.cards.drain(drain_start..).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn shuffled_deck() {
        let deck = Deck::new();
        let deck2 = Deck::new();

        assert_ne!(deck, deck2);
    }

    #[test]
    fn deck_60_cards() {
        for _ in 0..5 {
            assert_eq!(60, Deck::new().cards.len());
        }
    }

    #[test]
    fn deck_4_wizards() {
        let deck = Deck::new();
        let mut wizards = 0;

        for card in deck.cards {
            if let Card::SpecialCard(special_card) = card {
                if special_card == SpecialCard::Wizard {
                    wizards += 1;
                }
            }
        }

        assert_eq!(4, wizards);
    }

    #[test]
    fn deck_4_jesters() {
        let deck = Deck::new();
        let mut jesters = 0;

        for card in deck.cards {
            if let Card::SpecialCard(special_card) = card {
                if special_card == SpecialCard::Jester {
                    jesters += 1;
                }
            }
        }

        assert_eq!(4, jesters);
    }

    #[test]
    fn deck_no_normal_card_repeats() {
        let deck = Deck::new();
        let mut special_cards = 0;
        let mut normal_cards = HashSet::new();

        for card in deck.cards {
            match card {
                Card::NormalCard(normal_card) => {
                    normal_cards.insert(normal_card);
                }
                Card::SpecialCard(_) => {
                    special_cards += 1;
                }
            }
        }

        println!("{:#?}", normal_cards);

        assert_eq!(8, special_cards);
        assert_eq!(52, normal_cards.len());
    }

    #[test]
    fn to_and_from_json() {
        let deck = Deck::new();

        let string = serde_json::to_string(&deck).unwrap();
        let deck2 = serde_json::from_str(&string.to_string()).unwrap();

        println!("{}", string);
        assert_eq!(deck, deck2);
    }

    #[test]
    fn deal() {
        let mut deck = Deck::new();

        // Request 0 cards
        assert_eq!(0, deck.deal(0).unwrap().len());
        assert_eq!(60, deck.cards.len());

        assert_eq!(5, deck.deal(5).unwrap().len());
        assert_eq!(55, deck.cards.len());

        assert_eq!(7, deck.deal(7).unwrap().len());
        assert_eq!(48, deck.cards.len());

        assert_eq!(12, deck.deal(12).unwrap().len());
        assert_eq!(36, deck.cards.len());

        // Request too many cards
        assert!(deck.deal(100).is_err());

        // Deal all of the cards
        assert_eq!(36, deck.deal(36).unwrap().len());
        assert_eq!(0, deck.cards.len());

        // Request cards from an empty deck
        assert!(deck.deal(100).is_err());
    }
}
