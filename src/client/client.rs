use crate::cards::card::Card;
use crate::cards::suit::Suit;
use crate::network::action::Action;
use crate::network::network::{
    network_listener, serialize_and_write_to_network, wait_for_incoming_connection,
};
use local_ip_address::local_ip;
use serde_json::{json, Value};
use std::net::{TcpListener, TcpStream};

/// Connect to the server
///
/// # Returns
/// * Connection to read data from the server
/// * Connection to write data to the server
fn connect_to_server(host: &str, port: &str) -> std::io::Result<(TcpStream, TcpStream)> {
    // Server writer
    let mut server_writer = TcpStream::connect(format!("{}:{}", host, port))?;

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
    serialize_and_write_to_network(&mut server_writer, &port_json);

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

// TODO: Create a struct for the arguments to functions like bid containing the state, etc. so the
// client interfaces don't have to parse them
pub trait Client {
    /// Create a client
    fn new() -> Self;

    /// Bid for this hand
    fn bid(&mut self, json: &Value) -> u8;

    /// Picks a trump suit
    fn choose_trump(&mut self, json: &Value) -> Suit;

    /// Pick a card from "playable_cards"
    fn play_card(&mut self, json: &Value) -> Card;

    fn client(&mut self, host: &str, port: &str) -> std::io::Result<()> {
        println!("Connecting to the server");
        let (mut server_reader_stream, mut server_writer_stream) = connect_to_server(host, port)?;
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
                        serialize_and_write_to_network(&mut server_writer_stream, &bid_json);
                    }
                    ChooseTrump => {
                        let trump_suit = self.choose_trump(&json);
                        println!("Randomly picking trump: {:?}", trump_suit);
                        let trump_json = json!({
                            "action": Action::ChooseTrump,
                            "trump": trump_suit,
                        });
                        serialize_and_write_to_network(&mut server_writer_stream, &trump_json);
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
                        serialize_and_write_to_network(
                            &mut server_writer_stream,
                            &played_card_json,
                        );
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
