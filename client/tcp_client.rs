use std::{
    io::{Write, stdout},
    net::{TcpStream},
    thread,
};

use colored::{
    *
};

pub struct ServerSettings {
    pub ip: String,
    pub port: String,
}

pub fn client(settings: &ServerSettings) {
    let mut username_input: String = String::new();
    let stdin = std::io::stdin();
    print!("{} ",
            "Enter your username:".bold().blue(),
    );
    stdout().flush().unwrap();
    stdin
        .read_line(&mut username_input)
        .expect("Failed to read from stdin");

    let mut stream = loop {
        match TcpStream::connect(format!("{}:{}", settings.ip, settings.port)) {
            Ok(stream) => {
                println!("{}",
                         "Successfully connected to server".green(),
                );
                break stream;
            }
            Err(e) => {
                println!("{} {}",
                        "Failed to connect:".italic().red(),
                         e.to_string().red(),
                );
            }
        }
        thread::sleep(std::time::Duration::from_secs(3));
    };

    stream
        .write(username_input.as_bytes())
        .expect("Failed to write to server");

    loop {
        let mut user_input = String::new();
        let stdin = std::io::stdin();
        print!("{} ",
                "Enter your message:".bold().blue(),
        );
        stdout().flush().unwrap();
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read from stdin");

        stream
            .write(user_input.as_bytes())
            .expect("Failed to write to server");
    }
}