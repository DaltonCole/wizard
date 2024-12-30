use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Club,
    Diamond,
    Spade,
    Heart,
}
