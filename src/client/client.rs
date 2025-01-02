use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct HumanClient;

impl HumanClient {
    fn send_data(data: &Card) -> std::io::Result<()> {
        let mut stream = TcpStream::connect("0.0.0.0:7878")?;
        let serialized = serde_json::to_string(data).unwrap();
        stream.write_all(serialized.as_bytes())?;
        println!("Client sent {:?}", data);
        Ok(())
    }

    pub fn client(&mut self) -> std::io::Result<()> {
        let mut stream = TcpStream::connect("0.0.0.0:7878")?;
        println!("Connected to the server.");

        let mut stream_clone = stream.try_clone()?;

        // Listen to server
        thread::spawn(move || {
            // TODO: Make like server
            let mut buffer = [0; 512];
            loop {
                match stream_clone.read(&mut buffer) {
                    Ok(0) => {
                        println!("Server disconnected!");
                        break;
                    }
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Received from server: {}", response);
                    }
                    Err(e) => {
                        eprintln!("Error reading from server: {}", e);
                        break;
                    }
                }
            }
        });

        let tmp = serde_json::to_string(&Card::NormalCard(NormalCard {
            suit: Suit::Heart,
            rank: Rank::Two,
        }))
        .unwrap();
        println!("{}", tmp);

        // Send to server
        'server_send: loop {
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

            let serialized = serde_json::to_string(&card).unwrap();

            // Send data in chunks
            let chunk_size = 1024;

            for chunk in serialized.as_bytes().chunks(chunk_size) {
                if let Err(e) = stream.write_all(chunk) {
                    eprintln!("Error sending chunk: {}", e);
                    continue 'server_send;
                }
            }
        }
    }
}
