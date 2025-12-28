#![allow(unused)]
use rand::{Rng, distr::Alphanumeric};
use std::{env, io};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = TcpListener::bind("127.0.0.1:8080").expect("Couldn't bind");

    println!("server listening on 127.0.0.1:8080");

    match server.accept() {
        Ok((stream, addr)) => {
            println!("{:?}", addr);
            handle_client(stream);
        },
        Err(e) => {
            eprintln!("Client terminated request {:?}", e)
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to stream");
        if bytes_read == 0 {
            println!("terminated connection");
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received {}", message);
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't receive user input");
        stream.write_all(input.trim().as_bytes());
    }
}
