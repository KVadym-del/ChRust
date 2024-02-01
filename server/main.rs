mod tcp_server;

fn main() {
    let settings = &tcp_server::ServerSettings {
        ip: "127.0.0.1".to_owned(),
        port: "8081".to_owned(),
    };

    tcp_server::server(settings);
}