use chathost::config;

fn main() {
    println!("Client Started");
    let host = config::get_host();
    let port = config::get_port();
    println!("Host {}, Port {}", host, port);
}
