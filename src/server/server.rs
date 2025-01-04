use crate::game::wizard::WizardGame;
use crate::network::action::Action;
use crate::network::network::{network_listener, network_writer, wait_for_incoming_connection};
use serde_json::json;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server;

impl Server {
    /// Connect a write connection to the client
    ///
    /// This is a blocking function that waits for the client to send a Action::Connect over the
    /// given stream
    fn create_client_write_connection(client_read_stream: &mut TcpStream) -> TcpStream {
        loop {
            if let Ok((action, json)) = network_listener(client_read_stream) {
                if action == Action::Connect {
                    let host: String = serde_json::from_value(json["host"].clone()).unwrap();
                    let port = json["port"].as_u64().unwrap();
                    let mut client_write_stream =
                        TcpStream::connect(format!("{}:{}", host, port)).unwrap();

                    // Send message to client confirming the connection
                    let confirmation_msg = json!({
                        "action": Action::Confirmation,
                        "msg": "Server write connection established",
                    });
                    let serialized = serde_json::to_vec(&confirmation_msg).unwrap();

                    network_writer(&mut client_write_stream, serialized);

                    println!("Client connected on Port: {}", port);
                    return client_write_stream;
                }
            }
        }
    }

    pub fn start_server(&mut self, num_players: usize) {
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server running");

        // Wait for players to connect
        let mut player_read_streams = Vec::new();
        let mut player_write_streams = Vec::new();

        println!("Waiting for players to connect");
        while player_read_streams.len() != num_players {
            if let Ok(stream) = wait_for_incoming_connection(&listener) {
                player_read_streams.push(stream);
                println!(
                    "Player {} of {} connected!",
                    player_read_streams.len(),
                    num_players
                );
            }
        }

        // Create write connection to clients
        for mut player_read_stream in player_read_streams.iter_mut() {
            player_write_streams.push(Server::create_client_write_connection(
                &mut player_read_stream,
            ));
        }

        // Start game
        let mut game =
            WizardGame::new(num_players, player_read_streams, player_write_streams).unwrap();
        thread::sleep(std::time::Duration::from_millis(100)); // Delay helps clients connect to
                                                              // server properly
        game.play_game();
    }
}

mod tests {
    use super::*;
    use crate::client::random_client::RandomClient;
    use std::io::Read;

    #[test]
    fn full_game_with_3_clients() {
        let num_players = 4;
        // Start server
        let server_thread = thread::spawn(move || {
            let mut server = Server {};
            server.start_server(num_players);
        });

        // Let the server start up
        thread::sleep(std::time::Duration::from_millis(100));

        // Start num_players clients
        let mut client_threads = Vec::new();
        for _ in 0..num_players {
            thread::sleep(std::time::Duration::from_millis(100));
            client_threads.push(thread::spawn(move || {
                let mut client = RandomClient::new("0.0.0.0", "7878");
                if let Err(e) = client.client() {
                    panic!("Error occurred: {}", e);
                }
            }));
        }

        println!("Server Joining");
        server_thread.join().unwrap();
        println!("Server Joined");
        for client_thread in client_threads {
            client_thread.join().unwrap();
        }
        println!("client Joined");
    }
}
