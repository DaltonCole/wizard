use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Card {
    NormalCard(NormalCard),
    SpecialCard(SpecialCard),
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ser_str = match self {
            Card::SpecialCard(special_card) => match special_card {
                SpecialCard::Wizard => "Wizard".to_string(),
                SpecialCard::Jester => "Jester".to_string(),
            },
            Card::NormalCard(normal_card) => {
                format!("{}", normal_card)
            }
        };
        serializer.serialize_str(ser_str.as_str())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CardVisitor;

        impl<'de> Visitor<'de> for CardVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A string representing a card")
            }

            fn visit_str<E>(self, value: &str) -> Result<Card, E>
            where
                E: de::Error,
            {
                match value {
                    "Wizard" => Ok(Card::SpecialCard(SpecialCard::Wizard)),
                    "Jester" => Ok(Card::SpecialCard(SpecialCard::Jester)),
                    "AC" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Ace,
                    })),
                    "2C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Two,
                    })),
                    "3C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Three,
                    })),
                    "4C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Four,
                    })),
                    "5C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Five,
                    })),
                    "6C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Six,
                    })),
                    "7C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Seven,
                    })),
                    "8C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Eight,
                    })),
                    "9C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Nine,
                    })),
                    "10C" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Ten,
                    })),
                    "JC" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Jack,
                    })),
                    "QC" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Queen,
                    })),
                    "KC" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::King,
                    })),

                    "AD" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Ace,
                    })),
                    "2D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Two,
                    })),
                    "3D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Three,
                    })),
                    "4D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Four,
                    })),
                    "5D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Five,
                    })),
                    "6D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Six,
                    })),
                    "7D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Seven,
                    })),
                    "8D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Eight,
                    })),
                    "9D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Nine,
                    })),
                    "10D" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Ten,
                    })),
                    "JD" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Jack,
                    })),
                    "QD" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::Queen,
                    })),
                    "KD" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::King,
                    })),

                    "AS" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Ace,
                    })),
                    "2S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Two,
                    })),
                    "3S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Three,
                    })),
                    "4S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Four,
                    })),
                    "5S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Five,
                    })),
                    "6S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Six,
                    })),
                    "7S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Seven,
                    })),
                    "8S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Eight,
                    })),
                    "9S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Nine,
                    })),
                    "10S" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Ten,
                    })),
                    "JS" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Jack,
                    })),
                    "QS" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Queen,
                    })),
                    "KS" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::King,
                    })),

                    "AH" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Ace,
                    })),
                    "2H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Two,
                    })),
                    "3H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Three,
                    })),
                    "4H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Four,
                    })),
                    "5H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Five,
                    })),
                    "6H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Six,
                    })),
                    "7H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Seven,
                    })),
                    "8H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Eight,
                    })),
                    "9H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Nine,
                    })),
                    "10H" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Ten,
                    })),
                    "JH" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Jack,
                    })),
                    "QH" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::Queen,
                    })),
                    "KH" => Ok(Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::King,
                    })),

                    _ => Err(de::Error::unknown_variant(value, &["Card"])),
                }
            }
        }

        deserializer.deserialize_str(CardVisitor)
    }
}
