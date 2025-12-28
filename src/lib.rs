#![allow(unused)]
use std::error::Error;
use std::io::prelude::*;
use rand::{distr::Alphanumeric, Rng};

pub enum Command {
    Send,
    Receive,
}

pub struct Config {
    command: Command,
    files: Option<Vec<String>>
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please specify a command");
        }

        match args[1].as_str() {
            "send" => {
                if args.len() < 3 {
                    Err("Invalid arguments: select files for transfer")
                } else {
                    Ok(Config { command: Command::Send, files: Some(args[1..].to_vec()) })
                }
            }

            "receive" => {
                Ok(Config { command: Command::Receive, files: None })
            }

            _ => {
                Err("Invalid arguments provided")
            }

        }
    }

    pub fn generate_hash() -> String {
        let hash: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        hash
    }
}

pub mod file_sharing{
    pub mod sender {

        use crate::{Command, Config};
        use std::net::{Ipv4Addr, TcpListener, TcpStream};
        use std::io::{Read, Write};
        use std::fs::File;
        use std::env;
        use std::process;
        use std::thread;

        pub fn send_files() -> Result<bool,&'static str> {
            let server_password = Config::generate_hash();

            let server = TcpListener::bind("0.0.0.0:8201");

        }

        pub fn validate_client(stream: &mut TcpStream, server_password: &String) -> Result<bool, &'static str>{
            let mut buffer = [0; 1024];
            let mut attempts = 3;
            loop {
                let n = stream.read(&mut buffer).unwrap();
                let password = String::from_utf8_lossy(&buffer[..n]);

                if password.trim() == *server_password {
                    return Ok(true)
                } else {
                    attempts -= 1;
                    let mut response_message: String = format!("incorrect password specified \nYou have {attempts} left");
                    stream.write_all(response_message.as_bytes());
                }

                if attempts <= 0 {
                    break;
                }
            }

            Err("Invalid connection failed to provide password")
        }

        pub fn generate_header(stream: &mut TcpStream, files: &[String]) -> Result<bool, &'static str>{

        }

        pub fn file_transfer(){}


    }

    pub mod receive_files {

    }

}

