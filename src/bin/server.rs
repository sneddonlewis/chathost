use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

use chathost::config;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    let mut from_client = String::from("From Client: ");
    while match stream.read(&mut data) {
        Ok(size) => {
            let message = match str::from_utf8(&data[0..size]) {
                Ok(v) => v,
                Err(_) => "Received corrupted data from client",
            };
            from_client.push_str(message);
            stream.write(message.as_bytes()).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    println!("Server Started");
    let address = config::get_address();

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
