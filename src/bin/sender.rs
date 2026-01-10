#![allow(unused)]
use cli_tool::{Command, Config, generate_hash};
use core::error;
use std::{env, process, thread};
use std::fs::File;
use std::io::{Read, BufRead, BufReader, Write, Error};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use spake2::{Ed25519Group, Identity, Password, Spake2};

struct FileData {
    file_name: String,
    file_size: u64,
    file_type: String,
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args);
}

fn send_files() -> Result<bool, &'static str> {
    let Password = generate_hash();

    let server = TcpListener::bind("127.0.0.1:8201");

    todo!()
}

fn encrypt_server(stream: &mut TcpStream, server_password: String) -> Result<String, dyn std::error::Error> {
    let (s1, outbound_msg)= Spake2::<Ed25519Group>::start_b(
        &Password::new(server_password.as_bytes()),
        &Identity::new(b"file-receiver"),
        &Identity::new(b"file-sender"),
        );

    stream.write_all(&outbound_msg).unwrap();

    let mut peer_msg = [0u8; 33];
    stream.read_exact(&mut peer_msg).unwrap();

    let shared_key = s1.finish(&peer_msg).unwrap();

    let key = String::from_utf8_lossy(&shared_key);
    

}





