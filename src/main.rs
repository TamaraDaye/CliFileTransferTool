#![allow(unused)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = TcpListener::bind("127.0.0.1:8080").expect("Couldn't bind");

    println!("server listening on 127.0.0.1:8080");

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }

            Err(e) => {
                eprintln!("Error {e} occured");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to stream");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request \n{}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Couldn't write response");
}
