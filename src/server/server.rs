use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use crate::network::action::Action;
use crate::network::network::{handle_incoming_connections, network_listener, network_writer};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct Server;

impl Server {
    fn convert_binary_into_json(data: &Vec<u8>) -> Result<Value, serde_json::Error> {
        serde_json::from_slice(&data)
    }

    /// Create a thread for writing to a particular client
    fn client_writer(host: &str, port: u64) -> mpsc::Sender<Vec<u8>> {
        let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        let (tx, rx) = mpsc::channel();

        network_writer(stream, rx);

        tx
    }

    pub fn start_server(&mut self, num_players: usize) {
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server running");

        let (tx, rx) = mpsc::channel();

        // Spawn thread to handle incoming connections
        handle_incoming_connections(listener, tx, num_players);

        /*
        while let Ok((client_id, message)) = rx.recv() {
            let deserialized: serde_json::Value = serde_json::from_slice(&message).unwrap();
            let port = deserialized["port"].as_u64().unwrap();
            let (tx, rx) = mpsc::channel();
            let mut stream = TcpStream::connect(format!("0.0.0.0:{}", port)).unwrap();
            network_writer(stream.try_clone().unwrap(), rx);

            println!("{}", port);
            break;
        }
        */

        let mut client_writters = HashMap::new();

        let mut data = Vec::new();
        while let Ok((client_id, message)) = rx.recv() {
            data.extend(message.iter());
            if let Ok((action, json)) = Action::serde_find_action(&data) {
                // Wait for clients to connect
                if let Action::Connect = action {
                    let port = json["port"].as_u64().unwrap();
                    let tx = Server::client_writer("0.0.0.0", port);
                    // Send message to client confirming the connection
                    let confirmation_msg = json!({
                        "action": Action::Confirmation,
                        "msg": "Server write connection established",
                    });
                    let serialized = serde_json::to_vec(&confirmation_msg).unwrap();
                    tx.send(serialized).unwrap();
                    client_writters.insert(client_id, tx);

                    println!("Client {} - Port: {}", client_id, port);
                }
                /*
                match action {
                    Action::Confirmation => {
                        let msg: String = serde_json::from_value(json["msg"].clone()).unwrap();
                        println!("Clinet {} - Msg: {}", client_id, msg);
                    }
                    Action::Connect => {
                        let port = json["port"].as_u64().unwrap();
                        let tx = Server::client_writer("0.0.0.0", port);
                        // Send message to client confirming the connection
                        let confirmation_msg = json!({
                            "action": Action::Confirmation,
                            "msg": "Server write connection established",
                        });
                        let serialized = serde_json::to_vec(&confirmation_msg).unwrap();
                        tx.send(serialized).unwrap();
                        client_writters.insert(client_id, tx);

                        println!("Client {} - Port: {}", client_id, port);
                    }
                    Action::GiveBid => {
                        todo!();
                    }
                    Action::RequestBid => {
                        eprintln!("Error: Client only action sent by: {}", client_id);
                    }
                    Action::Card => {
                        let card: Card = serde_json::from_value(json["card"].clone()).unwrap();
                        println!("Client {} - Card: {:?}", client_id, card);
                    }
                }
                */
                data.clear();
            }
        }
    }
}
