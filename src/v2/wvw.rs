// /v2/wvw[/*]
use crate::common::client::{Gw2ApiError, Gw2Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WvwRank {
    pub id: i32,                        // Unique WvW rank ID
    pub title: String,                  // Title of the WvW rank
    pub min_rank: i32                   // Minimum WvW level required to be this rank
}

impl WvwRank {
    pub fn new() -> WvwRank {
        Self { id: 0, title: String::from("not found"), min_rank: 0 }
    }

    pub async fn get(client: &Gw2Client, player_wvw_level: i32) -> Result<WvwRank, Gw2ApiError> {
        let mut player_rank = WvwRank::new();

        // Unfortunately, there is no other way to get the WvW rank at the moment.
        match client.request::<Vec<i32>>("v2/wvw/ranks").await {
            Ok(available_ranks) => {
                for rank in available_ranks {
                    match client.request::<WvwRank>(&format!("v2/wvw/ranks/{rank}")).await {
                        Ok(wvw_rank) => {
                            if wvw_rank.min_rank <= player_wvw_level {
                                player_rank = wvw_rank;
                            }
                            else {
                                break;
                            }
                        },
                        Err(_) => break
                    }
                }
                Ok(player_rank)
            },
            Err(err) => Err(err)
        }
    }
}