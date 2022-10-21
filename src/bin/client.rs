use chathost::config;

fn main() {
    println!("Client Started");
    let host = config::get_host();
    let port = option_env!("PORT").unwrap_or_else(|| "12345");
    println!("Host {}, Port {}", host, port);
}
