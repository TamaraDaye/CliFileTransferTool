#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use rand::{distr::Alphanumeric, Rng};

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

        Ok(Config {command, files})

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
            let args: Vec<String> = env::args().collect();

            let config = Config::new(&args).unwrap_or_else(|err| {
                eprintln!("incorrect arguments specified: {}", err);
                process::exit(1);
            });

            let config_password = Config::generate_hash();

            if let Command::Send = config.command {
                let server = TcpListener::bind("0.0.0.0:8201").unwrap();
                println!("Awaiting receiver\n on receiving end enter {config_password}");
                
                match server.accept() {
                    Ok((mut stream, _addr)) => {
                        validate_client(&mut stream, &config_password)?;
                    },
                    Err(e) => {
                        eprintln!("error encountered {e:?}")
                    }
                };
            }

            Ok(true)
        }

        pub fn validate_client(stream: &mut TcpStream, server_password: &String) -> Result<bool, &'static str>{
            let mut buffer = [0; 1024];
            let mut attempts = 3;
            loop {
                let n = stream.read(&mut buffer).unwrap();
                let password = String::from_utf8_lossy(&buffer);

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


    }

    pub mod receive_files {

    }

}

