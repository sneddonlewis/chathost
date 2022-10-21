pub fn get_host() -> &'static str {
    option_env!("HOSTNAME").unwrap_or_else(|| "localhost")
}

pub fn get_port() -> &'static str {
    option_env!("PORT").unwrap_or_else(|| "12345")
}
