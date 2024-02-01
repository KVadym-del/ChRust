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

pub struct Client {
    settings: ServerSettings,
}

impl Client {
    pub fn new(settings: ServerSettings) -> Self  {
        Client {
            settings,
        }
    }

    pub fn create_client(&self) -> String {
        return self.get_username();
    }

    fn get_username(&self) -> String {
        let mut username_input: String = String::new();
        print!("{} ",
               "Enter your username:".bold().blue(),
        );
        stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut username_input)
            .expect("Failed to read from stdin");

        return username_input;
    }

    pub fn connect(&self, user_name: &String) {
        let mut stream = loop {
            match TcpStream::connect(format!("{}:{}", self.settings.ip, self.settings.port)) {
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
            .write(user_name.as_bytes())
            .expect("Failed to write to server");

        self.loop_send(&mut stream);
    }

    fn loop_send(&self, stream: &mut TcpStream) {
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
}