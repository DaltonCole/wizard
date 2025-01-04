use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use crate::network::action::Action;
use crate::network::network::{network_listener, network_writer, wait_for_incoming_connection};
use local_ip_address::local_ip;
use rand::Rng;
use serde_json::{json, Value};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;

pub struct RandomClient {
    host: String,
    port: String,
    rng: rand::rngs::ThreadRng,
}

impl RandomClient {
    pub fn new(host: &str, port: &str) -> RandomClient {
        let mut rng = rand::thread_rng();

        RandomClient {
            host: host.to_string(),
            port: port.to_string(),
            rng,
        }
    }
    fn send_data(&self, data: &Card) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        let serialized = serde_json::to_string(data).unwrap();
        stream.write_all(serialized.as_bytes())?;
        println!("Client sent {:?}", data);
        Ok(())
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

        /*
        let tmp = serde_json::to_string(&Card::NormalCard(NormalCard {
            suit: Suit::Heart,
            rank: Rank::Two,
        }))
        .unwrap();
        println!("{}", tmp);

        // Send to server
        'server_send: loop {
            if let Ok((_, msg)) = server_rx.try_recv() {
                println!("{:?}", String::from_utf8(msg));
            }
            print!("Enter card: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();

            let card: Card = match serde_json::from_str(trimmed) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error sending to server: {}", e);
                    continue;
                }
            };

            let card_json = json!({
                "action": Action::Card,
                "card": card,
            });
            let serialized = serde_json::to_vec(&card_json).unwrap();

            // Send data in chunks
            server_tx.send(serialized);
        }
        */
        Ok(())
    }
}
