// /v2/pvp[/*]
use crate::common::client::{Gw2ApiError, Gw2Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PvpStats {
    pub pvp_rank: i32,               // The player's PvP rank
    pub pvp_rank_rollovers: i32,     // The number of times the player has levelled up since reaching the max level
    pub aggregate: Aggregate         // Aggregated stats of all matches ever played
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aggregate {
    pub wins: i32,                   // The wins for the category
    pub losses: i32,                 // The losses for the category
    pub desertions: i32,             // The desertions for the category
    pub byes: i32,                   // The byes for the category
    pub forfeits: i32,               // The forfeits for the category
}

impl PvpStats {
    pub async fn get(client: &Gw2Client) -> Result<PvpStats, Gw2ApiError> {
        client.request("v2/pvp/stats").await
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PvpRank {
    pub id: i32,                     // The id of the PvP rank
    pub finisher_id: i32,            // The id of the unlocked finisher (see /v2/finishers)
    pub name: String                 // The name for the PvP rank
}

impl PvpRank {
    pub async fn get(client: &Gw2Client, player_pvp_rank: i32) -> Result<PvpRank, Gw2ApiError> {
        let mapped_rank = (player_pvp_rank / 10) + 1;
        client.request(&format!("v2/pvp/ranks/{}", mapped_rank)).await
    }
}