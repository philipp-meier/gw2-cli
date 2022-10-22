// /v2/characters[/*]
use crate::client::{Gw2ApiError, Gw2Client};
use crate::common::stats::{print_stats, StatsRow};
use crate::common::utils::get_age_from_create_date;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterCore {
    pub name: String,           // The character's name
    pub race: String,           // The character's race (Asura, Charr,...)
    pub gender: String,         // The character's gender
    pub profession: String,     // The character's profession
    pub level: i32,             // Level (1-80)
    pub age: i32,               // Amount of seconds this character was played
    pub deaths: i32,            // The amount of times this character has been defeated
    pub created: DateTime<Utc>, // ISO 8601 representation of the character's creation time
}

impl CharacterCore {
    pub async fn get_character(
        name: &str,
        client: &Gw2Client,
    ) -> Result<CharacterCore, Gw2ApiError> {
        client
            .request(&format!("v2/characters/{}/core", encode(&name)))
            .await
    }

    pub async fn get_characters(client: &Gw2Client) -> Result<Vec<String>, Gw2ApiError> {
        client.request::<Vec<String>>("v2/characters").await
    }

    pub async fn get_oldest_character(client: &Gw2Client) -> Result<CharacterCore, Gw2ApiError> {
        let mut oldest_character: CharacterCore = CharacterCore {
            name: "".to_string(),
            race: "".to_string(),
            gender: "".to_string(),
            profession: "".to_string(),
            level: 0,
            age: 0,
            deaths: 0,
            created: Default::default(),
        };

        match CharacterCore::get_characters(&client).await {
            Ok(character_names) => {
                for name in character_names {
                    match CharacterCore::get_character(&name, &client).await {
                        Ok(character) => {
                            if character.age >= oldest_character.age {
                                oldest_character = character;
                            }
                        }
                        Err(_) => break,
                    }
                }

                Ok(oldest_character)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn print_character_stats(name: &str, client: &Gw2Client) -> () {
        match CharacterCore::get_character(name, client).await {
            Ok(character) => {
                let mut rows = Vec::<StatsRow>::new();
                rows.push(StatsRow::new("Title", character.name));
                rows.push(StatsRow::new(
                    "Separator",
                    String::from("-----------------------"),
                ));
                rows.push(StatsRow::new("Race", character.race));
                rows.push(StatsRow::new("Gender", character.gender));
                rows.push(StatsRow::new("Profession", character.profession));
                rows.push(StatsRow::new("Level", character.level.to_string()));
                rows.push(StatsRow::new("Deaths", character.deaths.to_string()));
                rows.push(StatsRow::new(
                    "Age",
                    get_age_from_create_date(character.created),
                ));
                print_stats(rows);
            }
            Err(err) => println!("{}", err.error),
        }
    }
}

