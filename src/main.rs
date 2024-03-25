use datafusion_sprox::Result;
use datafusion_sprox::SproxProvider;
use hyper::server::Server;
use s3s::service::{S3Service, S3ServiceBuilder};
use std::path::PathBuf;
use std::{io::IsTerminal, net::TcpListener};
use tracing::info;

#[derive(Debug)]
struct SproxConf {
    // Server and Host
    listen_host: String,
    port: u16,
    // Auth
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    // Data
    data_directory: PathBuf,
    // Cache
}

impl SproxConf {
    fn new() -> Self {
        Self {
            listen_host: String::from("127.0.0.1"),
            port: 8080,
            access_key_id: Some(String::from("A")),
            secret_access_key: Some(String::from("B")),
            data_directory: "./data".into(),
        }
    }
}

fn main() -> Result {
    let conf = SproxConf::new();
    init(conf);
    runserver(conf)
}

fn init(conf: SproxConf) {
    // Init
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::from_default_env();
    let enable_color = std::io::stdout().is_terminal();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .with_ansi(enable_color)
        .init();
    info!("Initialization complete");
}

#[tokio::main]
async fn runserver(conf: SproxConf) -> Result {
    // Setup Sprox Provider
    let provider = SproxProvider::new(conf.data_directory);
    // Setup S3 Service
    let service: S3Service = {
        let mut b: S3ServiceBuilder = S3ServiceBuilder::new(provider);
        b.build()
    };

    // Run server
    let listener = TcpListener::bind((conf.listen_host, conf.port))?;
    let local_addr = listener.local_addr();
    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());
    server.with_graceful_shutdown(shutdown()).await?;
    Ok(())
}

async fn shutdown() {
    // TODO: Add more shutdown sigs
    let _ = tokio::signal::ctrl_c().await;
}
