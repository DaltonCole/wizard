use crate::network::action::Action;
use anyhow::{bail, Result};
use serde_json::Value;
use std::io::{Read, Result as IoResult, Write};
use std::net::{TcpListener, TcpStream};

/// Wait for an incoming connection
///
/// This is a blocking operation
pub fn wait_for_incoming_connection(listener: &TcpListener) -> IoResult<TcpStream> {
    match listener.accept() {
        Ok((socket, addr)) => {
            println!("New client: {addr:?}");
            Ok(socket)
        }
        Err(e) => Err(e),
    }
}

/// Spawn a thread that will listen to incoming network connections over the stream
/// Listen for data on a stream
///
/// This is a blocking operation. Program will wait until a serde_json-able piece of data is sent
/// over the stream
pub fn network_listener(stream: &mut TcpStream) -> Result<(Action, Value)> {
    let mut buffer = [0; 1024];
    let mut data: Vec<u8> = Vec::new();

    loop {
        // Read n bytes
        match stream.read(&mut buffer) {
            Ok(0) => {
                bail!("Client disconnected");
            }
            Ok(bytes_read) => {
                data.extend(&buffer[..bytes_read]);
                if let Ok((action, json)) = Action::serde_find_action(&data) {
                    return Ok((action, json));
                }
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                continue;
            }
        };
    }
}

/// Write data to a stream
pub fn network_writer(stream: &mut TcpStream, data: Vec<u8>) {
    let chunk_size = 1024;

    for chunk in data.as_slice().chunks(chunk_size) {
        if let Err(e) = stream.write_all(chunk) {
            eprintln!("Error sending chunk: {}", e);
            break;
        }
    }
}
