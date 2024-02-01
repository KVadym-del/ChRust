mod tcp_client;

fn main() {
    let settings = tcp_client::ServerSettings {
        ip: "127.0.0.1".to_owned(),
        port: "8081".to_owned(),
    };

    let client = tcp_client::Client::new(settings);
    let client_name = client.create_client();
    client.connect(&client_name);
}