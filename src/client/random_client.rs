use crate::cards::card::Card;
use crate::cards::suit::Suit;
use crate::network::action::Action;
use crate::network::network::{network_listener, network_writer, wait_for_incoming_connection};
use local_ip_address::local_ip;
use rand::Rng;
use serde_json::{json, Value};
use std::net::{TcpListener, TcpStream};
use strum::IntoEnumIterator;

pub struct RandomClient {
    host: String,
    port: String,
    rng: rand::rngs::ThreadRng,
}

impl RandomClient {
    /// Create a new random client
    pub fn new(host: &str, port: &str) -> RandomClient {
        let rng = rand::thread_rng();

        RandomClient {
            host: host.to_string(),
            port: port.to_string(),
            rng,
        }
    }

    /// Connect to the server
    ///
    /// # Returns
    /// * Connection to read data from the server
    /// * Connection to write data to the server
    fn connect_to_server(&self) -> std::io::Result<(TcpStream, TcpStream)> {
        // Server writer
        let mut server_writer = TcpStream::connect(format!("{}:{}", self.host, self.port))?;

        // Server listener
        let listener = TcpListener::bind("0.0.0.0:0")?; // Let OS decide port
        let port = listener.local_addr()?.port();
        let host = local_ip().unwrap();

        // Send over port to server
        let port_json = json!({
            "action": Action::Connect,
            "port": port,
            "host": host,
        });
        let serialized = serde_json::to_vec(&port_json).unwrap();
        network_writer(&mut server_writer, serialized);

        // Wait for incoming connection
        loop {
            match wait_for_incoming_connection(&listener) {
                Ok(server_reader) => {
                    return Ok((server_reader, server_writer));
                }
                Err(e) => {
                    eprintln!("Error in server read connection: {}", e);
                }
            }
        }
    }

    /// Send JSON to server
    fn network_writer(stream: &mut TcpStream, value: &Value) {
        let serialized = serde_json::to_vec(&value).unwrap();
        network_writer(stream, serialized);
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

    pub fn client(&mut self) -> std::io::Result<()> {
        println!("Connecting to the server");
        let (mut server_reader_stream, mut server_writer_stream) = self.connect_to_server()?;
        println!("Read/Write connection to the server established.");

        loop {
            use crate::network::action::Action::*;

            if let Ok((action, json)) = network_listener(&mut server_reader_stream) {
                match action {
                    Bid => {
                        let bid = self.bid(&json);
                        println!("Randomly bidding: {}", bid);
                        let bid_json = json!({
                            "action": Action::Bid,
                            "bid": bid,
                        });
                        RandomClient::network_writer(&mut server_writer_stream, &bid_json);
                    }
                    ChooseTrump => {
                        let trump_suit = self.choose_trump(&json);
                        println!("Randomly picking trump: {:?}", trump_suit);
                        let trump_json = json!({
                            "action": Action::ChooseTrump,
                            "trump": trump_suit,
                        });
                        RandomClient::network_writer(&mut server_writer_stream, &trump_json);
                    }
                    Confirmation => {
                        println!(
                            "Read connection from the server established. Json: {:#?}",
                            json
                        );
                    }
                    Connect => {
                        todo!();
                    }
                    Card => {
                        todo!();
                    }
                    EndGame => {
                        println!("Game has ended. Final Game State: {:#?}", json);
                        break;
                    }
                    PlayCard => {
                        let played_card = self.play_card(&json);
                        println!("Randomly playing: {:?}", played_card);
                        let played_card_json = json!({
                            "action": Action::PlayCard,
                            "played_card": played_card,
                        });
                        RandomClient::network_writer(&mut server_writer_stream, &played_card_json);
                    }
                    StartGame => {
                        println!("Starting the game. Initial game state: {:#?}", json);
                    }
                }
            }
        }

        Ok(())
    }
}
