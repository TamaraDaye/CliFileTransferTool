#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub enum Command {
    Send,
    Receive,
}

pub struct Config {
    command: Command,
    files: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please specify a file to send");
        }

        let files: Vec<String> = args[2..].to_vec();

        let mut command: Command  = match args[1].as_str() {
            "send" => {Command::Send},
            "receive" => {Command::Receive},
            _ => {return Err("Please specify a command");}
        };

        Ok(Config { command, files })

    }

}

pub mod file_sharing{
    pub mod sender {

        use crate::Config;
        use std::net::{Ipv4Addr, TcpListener, TcpStream};
        use std::io::{Read, Write};
        use std::fs::File;
        use std::thread;

        pub fn send_files (config: Config) -> Result<_,_> {
            let server = TcpListener::bind("0.0.0.0:8201").unwrap();
        }

        pub fn validate_client(mut stream: TcpStream) -> Result<bool, &'static str>{
            let mut buffer = [0; 1024];
            stream.read(&mut buffer);
            let password = String::from_utf8_lossy(&buffer[..]);
        }
    }

    pub mod receive_files {}
}
