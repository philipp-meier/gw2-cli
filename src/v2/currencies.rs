// /v2/currencies[/*]
use crate::client::{Gw2ApiError, Gw2Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Currency {
    pub id: i32,                     // Unique currency ID
    pub name: String,                // Currency name
    pub description: String,         // Description of the currency
    pub icon: String,                // URL to an icon for the currency
    pub order: i32,                  // Number that can be used to sort the list of currencies
}

impl Currency {
    pub async fn get(client: &Gw2Client, currency_id: i32) -> Result<Currency, Gw2ApiError> {
        match client.request::<Vec<World>>(&format!("v2/currencies?ids={}", currency_id)).await {
            Ok(currencies) => Ok(currencies.first().cloned().expect("Currency could not be resolved.")),
            Err(err) => Err(err)
        }
    }
}