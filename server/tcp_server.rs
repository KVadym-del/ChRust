use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    collections::HashMap,
};
use std::net::SocketAddr;

pub struct ServerSettings {
    pub ip: String,
    pub port: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    let mut amount_of_messages: i32 = 0;
    let mut client_mame: String = String::new();
    stream.set_read_timeout(Some(std::time::Duration::from_millis(100_000))).expect("Failed to set read timeout");
    match stream.read(&mut buf) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return;
            }
            client_mame = String::from_utf8_lossy(&buf[..bytes_read]).to_string().trim().parse().unwrap();
            println!("{} connected", client_mame);
            amount_of_messages += 1;
        },
        Err(e) => {
            println!("Failed to read from socket: {}", e);
            return;
        }
    }
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from socket");
        if bytes_read == 0 {
            return;
        }
        println!(
            "from {} received: {}",
            stream.peer_addr().unwrap(),
            String::from_utf8_lossy(&buf[..bytes_read])
        );
    }
}

pub fn server(settings: &ServerSettings) {
    let listener = TcpListener::bind(format!("{}:{}", settings.ip, settings.port))
        .expect("Could not bind");
    println!("Server listening on {}:{}", settings.ip, settings.port);
    let mut clients: HashMap<SocketAddr, String> = HashMap::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}