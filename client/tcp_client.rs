use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub struct ServerSettings {
    pub ip: String,
    pub port: String,
}

pub fn client(settings: &ServerSettings) {
    let mut username_input: String = String::new();
    let stdin = std::io::stdin();
    println!("Enter your username: ");
    stdin
        .read_line(&mut username_input)
        .expect("Failed to read from stdin");

    let mut stream = loop {
        match TcpStream::connect(format!("{}:{}", settings.ip, settings.port)) {
            Ok(stream) => {
                println!("Connected to server");
                break stream;
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        thread::sleep(std::time::Duration::from_secs(3));
    };

    stream
        .write(username_input.as_bytes())
        .expect("Failed to write to server");

    let mut buf = [0; 512];
    loop {
        let mut user_input = String::new();
        let stdin = std::io::stdin();
        println!("Enter your message: ");
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read from stdin");

        if user_input.eq("Exit") {
            break;
        }

        stream
            .write(user_input.as_bytes())
            .expect("Failed to write to server");
    }
}