fn main() {
    println!("Client Started");
    let host = option_env!("HOST").unwrap_or_else(|| "localhost");
    let port = option_env!("PORT").unwrap_or_else(|| "12345");
    println!("Host {}, Port {}", host, port);
}
