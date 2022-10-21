use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use chathost::config;

fn main() {
    println!("Client Started");
    let address = config::get_address();

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6];

            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
