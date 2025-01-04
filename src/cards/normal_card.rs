use crate::cards::rank::Rank;
use crate::cards::suit::Suit;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormalCard {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for NormalCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.rank {
                Rank::Ace => {
                    "A"
                }
                Rank::Two => {
                    "2"
                }
                Rank::Three => {
                    "3"
                }
                Rank::Four => {
                    "4"
                }
                Rank::Five => {
                    "5"
                }
                Rank::Six => {
                    "6"
                }
                Rank::Seven => {
                    "7"
                }
                Rank::Eight => {
                    "8"
                }
                Rank::Nine => {
                    "9"
                }
                Rank::Ten => {
                    "10"
                }
                Rank::Jack => {
                    "J"
                }
                Rank::Queen => {
                    "Q"
                }
                Rank::King => {
                    "K"
                }
            },
            match self.suit {
                Suit::Club => {
                    "C"
                }
                Suit::Diamond => {
                    "D"
                }
                Suit::Spade => {
                    "S"
                }
                Suit::Heart => {
                    "H"
                }
            }
        )
    }
}
