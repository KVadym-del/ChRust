mod tcp_client;

fn main() {
    let settings = &tcp_client::ServerSettings {
        ip: "127.0.0.1".to_owned(),
        port: "8081".to_owned(),
    };

    tcp_client::client(settings);
}