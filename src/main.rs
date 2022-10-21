use common::client;
use v2::account::Account;
use v2::worlds::World;
use std::env;
use clap::{Parser, Subcommand};

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
    pub mod characters;
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, value_parser, default_value = "en")]
    lang: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Character information
    Characters { command: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "Gw2Cli_ApiKey";
    let api_key = match env::var(name) {
        Ok(val) => val,
        Err(e) => panic!("${} is not set ({}).", name, e)
    };

    let cli = Cli::parse();
    let api_client = client::Gw2Client::new(api_key, cli.lang);

    match &cli.command {
        Some(c) => match c {
            Commands::Characters { command } => {
                if command == "list" {
                    match v2::characters::get_characters(&api_client).await {
                        Ok(characters) => {
                            for character in characters {
                                println!("{character}");
                            }
                        },
                        Err(e) => println!("{}", e.error)
                    }
                } else {
                    println!("Command not supported.");
                }
            }
        },
        None => common::stats::print(&api_client).await
    }

    Ok(())
}