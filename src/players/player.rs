use crate::cards::card::Card;
use crate::cards::suit::Suit;
use crate::network::action::Action;
use crate::network::network::{network_listener, network_writer};
use serde_json::{from_value, json, Value};
use std::net::TcpStream;
use std::sync::mpsc;

pub struct Player {
    pub score: i16,
    pub bid: Option<u8>,
    pub cards: Vec<Card>,
    pub tricks_taken: u8,
    client_listener: TcpStream,
    client_writer: TcpStream,
}

impl Player {
    pub fn new(client_listener: TcpStream, client_writer: TcpStream) -> Player {
        Player {
            score: 0,
            bid: None,
            cards: Vec::new(),
            tricks_taken: 0,
            client_listener,
            client_writer,
        }
    }

    fn network_writer(&mut self, value: &Value) {
        let serialized = serde_json::to_vec(value).unwrap();
        network_writer(&mut self.client_writer, serialized);
    }

    /// Inform clients that we are starting the game
    pub fn start_game(&mut self, game_state: &Value) {
        let send_start_game_action_json = json!({
            "action": Action::StartGame,
            "state": game_state,
        });
        self.network_writer(&send_start_game_action_json);
    }

    pub fn end_game(&mut self, game_state: &Value) {
        let send_end_game_action_json = json!({
            "action": Action::EndGame,
            "state": game_state,
        });
        self.network_writer(&send_end_game_action_json);
    }

    pub fn bid(&mut self, game_state: &Value) {
        // Send to server bid action + game state
        let send_bid_action_json = json!({
            "action": Action::Bid,
            "bid": 0,
            "state": game_state,
        });
        self.network_writer(&send_bid_action_json);

        // Receive bid from client
        loop {
            if let Ok((action, json)) = network_listener(&mut self.client_listener) {
                if action == Action::Bid {
                    self.bid = Some(json["bid"].as_u64().unwrap() as u8);
                    return;
                } else {
                    eprintln!(
                        "None bid action received during bidding phase. Action: {:?}",
                        action
                    );
                }
            }
        }
    }

    pub fn play_card(&mut self, game_state: &Value) -> Card {
        println!("{}", self.cards.len());
        let playable_cards = self.playable_cards(game_state);
        // TODO
        let played_card = playable_cards[0].clone();

        // Remove played card from hand
        match self.cards.iter().position(|card| *card == played_card) {
            Some(index) => self.cards.remove(index),
            None => panic!(
                "Played card is not in player's hand. Card: {:?}; Hand: {:?}",
                played_card, self.cards
            ),
        };

        played_card
    }

    /// List of playable cards given the current hand and what has been played
    fn playable_cards(&self, game_state: &Value) -> Vec<Card> {
        let mut playable_cards = Vec::new();
        let lead_suit =
            from_value::<Option<Suit>>(game_state.get("leading_suit").unwrap().clone()).unwrap();

        let has_lead_suit = self.cards.iter().any(|card| {
            if let Card::NormalCard(normal_card) = &card {
                if Some(normal_card.suit) == lead_suit {
                    return true;
                }
            }
            false
        });

        // If hand does not contain lead suit, they can play whatever card
        if !has_lead_suit {
            return self.cards.clone();
        }

        for card in self.cards.iter() {
            match card {
                Card::SpecialCard(_) => {
                    playable_cards.push(card.clone());
                }
                Card::NormalCard(normal_card) => {
                    if Some(normal_card.suit) == lead_suit {
                        playable_cards.push(card.clone());
                    }
                }
            };
        }

        playable_cards
    }

    pub fn won_trick(&mut self) {
        self.tricks_taken += 1;
    }

    /// Update the score and reset bid and tricks_taken
    /// # Panics
    /// If a player has cards remaining or if the player does not have a bid yet.
    pub fn update_score(&mut self) {
        match self.bid {
            Some(bid) => {
                if self.tricks_taken == bid {
                    self.score += 20 + (10 * self.tricks_taken as i16);
                } else {
                    self.score -= (bid as i8 - self.tricks_taken as i8).abs() as i16 * 10;
                }
            }
            None => panic!("Cannot update score before having a bid!"),
        }
        self.bid = None;
        self.tricks_taken = 0;
        if self.cards.len() != 0 {
            panic!("Cannot calculate score before all cards have been played!");
        }
    }

    /// Choose trump in the case of a wizard being trump
    pub fn choose_trump(&self, game_state: &Value) -> Suit {
        // TODO
        Suit::Spade
    }
}
