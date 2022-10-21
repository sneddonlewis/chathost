fn get_host() -> &'static str {
    option_env!("HOSTNAME").unwrap_or_else(|| "localhost")
}

fn get_port() -> &'static str {
    option_env!("PORT").unwrap_or_else(|| "12345")
}

pub fn get_address() -> String {
    format!("{}:{}", get_host().to_string(), get_port().to_string())
}
