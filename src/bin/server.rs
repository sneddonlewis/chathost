use chathost::config;

fn main() {
    println!("Server Started");
    let host = config::get_host();
    let port = config::get_port();
    println!("Server on host {}, port {}", host, port);
}
