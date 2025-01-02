use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use crate::network::network::{network_listener, network_writer};
use serde_json::Value;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct Server;

impl Server {
    /// Spawns a thread that listens to a specific client
    fn client_listener(id: usize, stream: TcpStream, tx: mpsc::Sender<(usize, Vec<u8>)>) {
        thread::spawn(move || network_listener(id, stream, tx));
    }

    /// Spawn a thread that will handle incoming connections up to N connections
    fn handle_incoming_connections(
        listener: TcpListener,
        tx: mpsc::Sender<(usize, Vec<u8>)>,
        n: usize,
    ) {
        thread::spawn(move || {
            for (id, stream) in listener.incoming().enumerate() {
                match stream {
                    Ok(stream) => {
                        Server::client_listener(id, stream, tx.clone());
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
                if id == n.saturating_sub(1) {
                    break;
                }
            }
        });
    }

    fn convert_binary_into_card(data: &Vec<u8>) -> Result<Card, serde_json::Error> {
        serde_json::from_slice(&data)
    }

    pub fn start_server(&mut self, num_players: usize) {
        let mut game = WizardGame::new(num_players).unwrap();

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server running");

        let (tx, rx) = mpsc::channel();

        // Spawn thread to handle incoming connections
        Server::handle_incoming_connections(listener, tx, num_players);

        let mut data = Vec::new();
        while let Ok((client_id, message)) = rx.recv() {
            data.extend(message.iter());
            if let Ok(card) = Server::convert_binary_into_card(&data) {
                println!("Client {}: {:?}", client_id, card);
                data.clear();
            }
        }
    }
}
