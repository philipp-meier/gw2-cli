use crate::common::client::{Gw2Client,Gw2ApiError};
use crate::common::utils::*;
use crate::v2::account::Wallet;
use crate::v2::pvp::{PvpRank, PvpStats};
use crate::v2::wvw::WvwRank;
use crate::v2::account::Account;
use crate::v2::worlds::World;
use crate::v2::characters;
use colored::Colorize;

pub struct StatsRow {
    title: String,
    text: String,
}

impl StatsRow {
    pub fn new(title: &str, text: String) -> StatsRow {
        Self {
            title: String::from(title),
            text,
        }
    }
}

pub async fn print(client: &Gw2Client) -> Result<(), Gw2ApiError>{
    let mut rows = Vec::<StatsRow>::new();

    match Account::get(&client).await {
        Ok(account) => {
            rows.push(StatsRow::new("Title", account.name));
            rows.push(StatsRow::new(
                "Separator",
                String::from("-----------------------"),
            ));
            rows.push(StatsRow::new(
                "Account Age",
                get_age_from_create_date(account.created),
            ));

            // Account create date
            rows.push(StatsRow::new("Created", account.created.to_string()));

            // (Home-)World of the player
            let world_row = match World::get(&client, account.world).await {
                Ok(world) => {
                    StatsRow::new("World", format!("{} ({})", world.name, world.population))
                }
                Err(_) => StatsRow::new("World", account.world.to_string()),
            };
            rows.push(world_row);

            // In-game currency (Gold, Silver, Copper)
            match Wallet::get(&client).await {
                Ok(wallets) => {
                    for wallet in wallets {
                        if wallet.id == 1 {
                            rows.push(StatsRow::new("Coins", format_coins(wallet.value)));
                            break;
                        }
                    }
                }
                Err(_) => rows.push(StatsRow::new("Coins", String::from("not found"))),
            }

            // PvP stats
            match PvpStats::get(&client).await {
                Ok(stats) => {
                    let rank = match PvpRank::get(&client, stats.pvp_rank).await {
                        Ok(rank) => rank.name,
                        Err(err) => String::from(err.error),
                    };
                    rows.push(StatsRow::new("PvP", format_pvp_stats_row(stats, rank)));
                }
                Err(_) => rows.push(StatsRow::new("PvP", String::from("not found"))),
            }

            // WvW stats
            match WvwRank::get(&client, account.wvw_rank).await {
                Ok(rank) => rows.push(StatsRow::new(
                    "WvW",
                    format_wvw_stats_row(account.wvw_rank, rank.title),
                )),
                Err(err) => rows.push(StatsRow::new("WvW", err.error)),
            }

            // Oldest character stats
            match characters::get_oldest_character(&client).await {
                Ok(oldest_char) => {
                    rows.push(StatsRow::new("Oldest char", format_char_row(oldest_char)))
                }
                Err(err) => rows.push(StatsRow::new("Oldest char", err.error)),
            }
        }
        Err(err) => return Err(err),
    };

    print_stats(rows);

    Ok(())
}

pub fn print_stats(stats_rows: Vec<StatsRow>) {
    let ascii_logo_rows = get_ascii_logo_rows();
    let stats_rows_length = stats_rows.len();
    for (i, ascii_line) in ascii_logo_rows.iter().enumerate() {
        let formatted_ascii_line = ascii_line.red().bold();

        if i > 0 && i - 1 < stats_rows_length {
            let stats_row_index = i - 1;
            let stats_row = &stats_rows[stats_row_index];

            if stats_row_index < 2 {
                let text = if stats_row_index == 0 {
                    stats_row.text.bold()
                } else {
                    stats_row.text.white()
                };
                println!("{}{:14}", formatted_ascii_line, text);
            } else {
                println!(
                    "{}{:14}{}",
                    formatted_ascii_line,
                    format!("{}:", stats_row.title).bold(),
                    stats_row.text
                );
            }
        } else {
            println!("{formatted_ascii_line}");
        }
    }
}

