use crate::cards::card::Card;
use crate::cards::suit::Suit;
use crate::client::client::Client;
use crate::network::action::Action;
use crate::network::network::{network_listener, network_writer, wait_for_incoming_connection};
use local_ip_address::local_ip;
use rand::Rng;
use serde_json::{json, Value};
use std::net::{TcpListener, TcpStream};
use strum::IntoEnumIterator;

pub struct RandomClient {
    rng: rand::rngs::ThreadRng,
}

impl Client for RandomClient {
    /// Create a new random client
    fn new() -> RandomClient {
        let rng = rand::thread_rng();

        RandomClient { rng }
    }

    /// Generates a random bid between 0 and the round number
    fn bid(&mut self, json: &Value) -> u8 {
        // Get round number
        let num_players = json["state"]["round"].as_u64().unwrap();
        // Make random bend
        self.rng.gen_range(0..=num_players) as u8
    }

    /// Picks a random trump suit
    fn choose_trump(&mut self, _: &Value) -> Suit {
        let suits: Vec<Suit> = Suit::iter().collect();
        let index = self.rng.gen_range(0..suits.len());
        suits[index]
    }

    /// Picks a random card from "playable_cards"
    fn play_card(&mut self, json: &Value) -> Card {
        // Get playable cards
        let playable_cards: Vec<Card> =
            serde_json::from_value(json["playable_cards"].clone()).unwrap();
        let index = self.rng.gen_range(0..playable_cards.len());
        playable_cards[index]
    }
}
