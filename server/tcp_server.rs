use std::{
    io::{Read},
    net::{TcpListener, TcpStream},
    thread,
};

use colored::{
    *
};

pub struct ServerSettings {
    pub ip: String,
    pub port: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    let mut client_mame: String = String::new();
    stream.set_read_timeout(Some(std::time::Duration::from_millis(100_000))).expect("Failed to set read timeout");
    match stream.read(&mut buf) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return;
            }
            client_mame = String::from_utf8_lossy(&buf[..bytes_read]).to_string().trim().parse().unwrap();
            println!("{} {}",
                     client_mame.italic().yellow(),
                     "connected".green(),
            );
        },
        Err(e) => {

            println!("{} {}",
                     "Failed to read from socket:".italic().red(),
                     e.to_string().red(),
            );
            return;
        }
    }
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from socket");
        if bytes_read == 0 {
            return;
        }
        print!(
            "{} => {}",
            client_mame.italic().yellow(),
            String::from_utf8_lossy(&buf[..bytes_read]).blue()
        );
    }
}

pub fn server(settings: &ServerSettings) {
    let listener = TcpListener::bind(format!("{}:{}", settings.ip, settings.port))
        .expect("Could not bind");
    println!("{} {}",
             "Server listening on".green(),
             format!("{}:{}", settings.ip, settings.port).bold().green(),
    );
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("{} {}",
                        "Unable to connect:".italic().red(),
                         e.to_string().red(),
                );
            }
        }
    }
}