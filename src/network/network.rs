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

/// Spawn a thread that will create a network listener for each incoming connection, up to N times
pub fn handle_incoming_connections(
    listener: TcpListener,
    tx: mpsc::Sender<(usize, Vec<u8>)>,
    n: usize,
) {
    thread::spawn(move || {
        for (id, stream) in listener.incoming().enumerate() {
            match stream {
                Ok(stream) => {
                    let tx_clone = tx.clone();
                    thread::spawn(move || network_listener(id, stream, tx_clone));
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
            if id == n.saturating_sub(1) {
                break;
            }
        }
    });
}

pub fn network_listener(id: usize, mut stream: TcpStream, tx: mpsc::Sender<(usize, Vec<u8>)>) {
    let mut buffer = [0; 1024];

    loop {
        // Read n bytes
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                tx.send((id, buffer[..bytes_read].into())).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                continue;
            }
        };
    }
}

pub fn network_writer(mut stream: TcpStream, rx: mpsc::Receiver<Vec<u8>>) {
    let chunk_size = 1024;

    while let Ok(message) = rx.recv() {
        for chunk in message.as_slice().chunks(chunk_size) {
            if let Err(e) = stream.write_all(chunk) {
                eprintln!("Error sending chunk: {}", e);
                break;
            }
        }
    }
}
