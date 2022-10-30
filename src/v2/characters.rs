// /v2/characters[/*]
use crate::common::client::{Gw2ApiError, Gw2Client};
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
    pub fn new() -> CharacterCore {
        Self {
            name: "".to_string(),
            race: "".to_string(),
            gender: "".to_string(),
            profession: "".to_string(),
            level: 0,
            age: 0,
            deaths: 0,
            created: Default::default(),
        }
    }

    pub async fn get(client: &Gw2Client, char_name: &str) -> Result<CharacterCore, Gw2ApiError> {
        client.request(&format!("v2/characters/{}/core", encode(&char_name))).await
    }

    pub async fn get_char_names(client: &Gw2Client) -> Result<Vec<String>, Gw2ApiError> {
        client.request::<Vec<String>>("v2/characters").await
    }
}

pub async fn handle_command(client: &Gw2Client, command: &str) -> Result<(), Gw2ApiError> {
    match command {
        "list" => print_characters(&client).await,
        _ => print_character_stats(&client, command).await
    }
}

pub async fn print_characters(client: &Gw2Client) -> Result<(), Gw2ApiError> {
    match CharacterCore::get_char_names(&client).await {
        Ok(characters) => {
            for character in characters {
                println!("{character}");
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
pub async fn print_character_stats(client: &Gw2Client, name: &str) -> Result<(), Gw2ApiError> {
    match CharacterCore::get(client, name).await {
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
        Err(err) => return Err(err),
    }
    Ok(())
}

pub async fn get_oldest_character(client: &Gw2Client) -> Result<CharacterCore, Gw2ApiError> {
    let mut oldest_character: CharacterCore = CharacterCore::new();

    match CharacterCore::get_char_names(&client).await {
        Ok(char_names) => {
            for char_name in char_names {
                match CharacterCore::get(&client, &char_name).await {
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