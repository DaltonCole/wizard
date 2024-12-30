use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
//use serde::de::{self, Deserialize, Deserializer, Visitor};
//use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
}
