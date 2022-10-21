pub fn get_host() -> &'static str {
    option_env!("HOST").unwrap_or_else(|| "localhost")
}
