#![allow(unused)]

use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to server");
    let mut buffer = [0; 1024];
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("invalid argument sent");
        stream.write_all(&input.trim().as_bytes());
        let n = stream.read(&mut buffer).expect("Error for server");
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("{response}");

        if response == "confirmed".to_string() {
            println!("ok");
            break
        }

    }
}
