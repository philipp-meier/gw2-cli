// /v2/worlds[/*]
use crate::common::client::{Gw2ApiError, Gw2Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct World {
    pub id: i32,            // Unique World ID
    pub name: String,       // World name
    pub population: String, // Population level (Low, Medium, High, VeryHigh, Full)
}

impl World {
    pub async fn get(client: &Gw2Client, world_id: i32) -> Result<World, Gw2ApiError> {
        match client
            .request::<Vec<World>>(&format!("v2/worlds?ids={world_id}"))
            .await
        {
            Ok(worlds) => Ok(worlds
                .first()
                .cloned()
                .expect("World could not be resolved.")),
            Err(err) => Err(err),
        }
    }
}

