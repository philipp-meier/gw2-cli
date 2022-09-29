// /v2/account[/*]
use crate::client::{Gw2ApiError, Gw2Client};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,                     // Account GUID
    pub name: String,                   // Unique account name with numerical suffix.
    pub age: i32,                       // Age of the account in seconds
    pub world: i32,                     // The id of the home world of the account (see /v2/worlds)
    pub guilds: Vec<String>,            // A list of guilds assigned to the account
    pub guild_leader: Vec<String>,      // A list of guilds the account is the leader of
    pub created: DateTime<Utc>,         // ISO-8601 standard timestamp for the account creation date
    pub access: Vec<String>,            // List of what content this account has access to (e.g. PoF)
    pub commander: bool,                // True, if the player has bought the commander tag
    pub fractal_level: i32,             // The account's personal fractal reward level
    pub daily_ap: i32,                  // The daily AP the account has
    pub monthly_ap: i32,                // The monthly AP the account has
    pub wvw_rank: i32,                  // The account's wvw rank
}

impl Account {
    pub async fn get(client: &Gw2Client) -> Result<Account, Gw2ApiError> {
        client.request("/v2/account").await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub id: i32,                        // The currency's ID (see /v2/currencies)
    pub value: i32                      // Amount of the currency
}

impl Wallet {
    pub async fn get(client: &Gw2Client) -> Result<Vec<Wallet>, Gw2ApiError> {
        client.request("v2/account/wallet").await
    }
}