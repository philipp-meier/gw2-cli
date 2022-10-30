use std::process;
use gw2cli::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem building the configuration: {err}");
        process::exit(1);
    });

    if let Err(e) = gw2cli::run(config).await {
        eprintln!("Application error: {}", e.error);
        process::exit(1);
    }

    Ok(())
}