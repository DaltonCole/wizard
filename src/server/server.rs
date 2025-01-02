use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct Server;

impl Server {
    pub fn start_server(&mut self, num_players: usize) {
        let mut game = WizardGame::new(num_players).unwrap();

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server running");

        let (tx, rx) = mpsc::channel();

        // Spawn thread to handle incoming connections
        thread::spawn(move || {
            for (id, stream) in listener.incoming().enumerate() {
                match stream {
                    Ok(stream) => {
                        let tx_clone = tx.clone();
                        thread::spawn(move || Server::handle_client(id, stream, tx_clone));
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        });

        while let Ok((client_id, message)) = rx.recv() {
            println!("Client {}: {:?}", client_id, message);
        }
    }

    fn handle_client(client_id: usize, mut stream: TcpStream, tx: mpsc::Sender<(usize, Card)>) {
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(bytes_read) => {
                    let received_data = &buffer[..bytes_read];
                    if let Ok(json_str) = std::str::from_utf8(received_data) {
                        if let Ok(message) = serde_json::from_str::<Card>(json_str) {
                            println!("Server received: {:?}", message);
                            tx.send((client_id, message)).unwrap();
                        }
                    }
                }
                Err(e) => eprintln!("Failed to read from client: {}", e),
            }
        }
    }
}
