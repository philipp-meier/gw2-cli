mod common;
mod v2;

use crate::common::client::{Gw2ApiError, Gw2Client};
use crate::v2::characters;
use clap::{Parser, Subcommand};
use std::env;

const ENV_API_KEY_NAME: &'static str = "Gw2Cli_ApiKey";

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, value_parser, default_value = "en")]
    lang: String,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Character information
    Characters { command: String },
}

pub struct Config {
    pub lang: String,
    pub command: Option<Commands>,
    pub api_key: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let api_key = match env::var(ENV_API_KEY_NAME) {
            Ok(val) => val,
            Err(_e) => {
                return Err("Environment variable \"{API_KEY_ENV_VAR_NAME}\" is not defined.")
            }
        };

        let cli: Cli = Cli::parse();
        Ok(Config {
            lang: cli.lang,
            command: cli.command,
            api_key,
        })
    }
}

pub async fn run(config: Config) -> Result<(), Gw2ApiError> {
    let client = Gw2Client::new(config.api_key, config.lang);
    match config.command {
        Some(command) => match command {
            Commands::Characters { command } => characters::handle_command(&client, &command).await,
        },
        None => common::stats::print(&client).await,
    }
}

