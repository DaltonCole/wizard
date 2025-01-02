use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use crate::network::network::{network_listener, network_writer};
use serde_json::Value;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct RandomClient {
    host: String,
    port: String,
}

impl RandomClient {
    pub fn new(host: &str, port: &str) -> RandomClient {
        RandomClient {
            host: host.to_string(),
            port: port.to_string(),
        }
    }
    fn send_data(&self, data: &Card) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        let serialized = serde_json::to_string(data).unwrap();
        stream.write_all(serialized.as_bytes())?;
        println!("Client sent {:?}", data);
        Ok(())
    }

    /// Spawns a thread that listens to the server
    ///
    /// # Returns
    /// A channel that receives binary data from the server
    fn server_listener(stream: TcpStream) -> mpsc::Receiver<(usize, Vec<u8>)> {
        let (tx, rx) = mpsc::channel();

        network_listener(0, stream, tx);

        rx
    }

    /// Spawns a thread that writes to the server
    ///
    /// # Returns
    /// A channel that can send binary data to the server
    fn server_writer(stream: TcpStream) -> mpsc::Sender<Vec<u8>> {
        let (tx, rx) = mpsc::channel();

        network_writer(stream, rx);

        tx
    }

    pub fn client(&mut self) -> std::io::Result<()> {
        let mut stream = TcpStream::connect("0.0.0.0:7878")?;
        println!("Connected to the server.");

        let server_rx = RandomClient::server_listener(stream.try_clone()?);
        let server_tx = RandomClient::server_writer(stream.try_clone()?);

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
            server_tx.send(serialized.as_bytes().into());
        }
    }
}
