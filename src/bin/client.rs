#![allow(unused)]

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to server");
    let request = "Hello, server".as_bytes();
    stream
        .write(request)
        .expect("Server couldn't receive request");
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("failed to receive response");
    let response = String::from_utf8_lossy(&buffer[..]);
    println!("{response}");
}
