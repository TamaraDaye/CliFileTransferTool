#![allow(unused)]

use spake2::{Ed25519Group, Identity, Password, Spake2};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

fn main() {
    let mut password = String::from("");
    io::stdin().read_line(&mut password);
    password.trim();

    let (s1, outbound_msg) = Spake2::<Ed25519Group>::start_a(
        &Password::new(password.as_bytes()),
        &Identity::new(b"file-client"),
        &Identity::new(b"file-server"),
    );

    let mut connection = TcpStream::connect("127.0.0.1:8201").unwrap();
    
    connection.write_all(&outbound_msg);

    let mut buffer = [0; 1024];

    let n = connection.read(&mut buffer).unwrap();

    let inbound_msg = String::from_utf8_lossy(&buffer[..n]);
}

