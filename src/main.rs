use clap::Parser;
use common::client;
use v2::account::Account;
use v2::worlds::World;

mod common {
    pub mod client;
    pub mod stats;
    pub mod utils;
}

mod v2 {
    pub mod account;
    pub mod worlds;
    pub mod pvp;
    pub mod wvw;
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, value_parser)]
    api_key: String,
    #[clap(short, long, value_parser, default_value = "en")]
    lang: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let api_client = client::Gw2Client::new(args.api_key, args.lang);

    common::stats::print(&api_client).await;

    Ok(())
}