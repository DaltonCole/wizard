use crate::cards::card::Card;
use crate::cards::normal_card::NormalCard;
use crate::cards::rank::Rank;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::game::wizard::WizardGame;
use crate::network::action::Action;
use crate::network::network::{handle_incoming_connections, network_listener, network_writer};
use serde_json::{json, Value};
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

    /// Connect to the server
    ///
    /// # Returns
    /// * Connection to read data from the server
    /// * Connection to write data to the server
    fn connect_to_server(
        &self,
    ) -> std::io::Result<(mpsc::Receiver<(usize, Vec<u8>)>, mpsc::Sender<Vec<u8>>)> {
        let (listener_tx, server_rx) = mpsc::channel();
        let (server_tx, writer_rx) = mpsc::channel();

        // Server listener
        let listener = TcpListener::bind("0.0.0.0:0")?; // Let OS decide port
        let port = listener.local_addr()?.port();
        handle_incoming_connections(listener, listener_tx, 1);

        // Server writer
        let mut stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        network_writer(stream.try_clone()?, writer_rx);

        // Send over port to server
        let port_json = json!({
            "action": Action::Connect,
            "port": port,
        });
        let serialized = serde_json::to_vec(&port_json).unwrap();
        server_tx.send(serialized);

        Ok((server_rx, server_tx))
    }

    pub fn client(&mut self) -> std::io::Result<()> {
        let (server_rx, server_tx) = self.connect_to_server()?;
        println!("Connected to the server.");

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
    }
}
