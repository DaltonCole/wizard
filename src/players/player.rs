use crate::cards::card::Card;

pub struct Player {
    pub score: i16,
    pub bid: u8,
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            score: 0,
            bid: 0,
            cards: Vec::new(),
        }
    }
}
