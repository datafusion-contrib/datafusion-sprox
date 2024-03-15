use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub data_directory: PathBuf,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            data_directory: PathBuf::new().join("/tmp/sprox/data/"),
        }
    }
}
