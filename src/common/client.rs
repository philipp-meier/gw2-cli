use reqwest::header::{ACCEPT_LANGUAGE, AUTHORIZATION};
use serde::de::DeserializeOwned;

pub struct Gw2Client {
    api_key: String,
    lang: String,
    req_client: reqwest::Client,
}

impl Gw2Client {
    pub fn new(api_key: String, lang: String) -> Gw2Client {
        Self {
            api_key,
            lang,
            req_client: reqwest::Client::new(),
        }
    }

    pub async fn request<T>(&self, endpoint: &str) -> Result<T, Gw2ApiError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .req_client
            .get(format!("https://api.guildwars2.com/{endpoint}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(ACCEPT_LANGUAGE, self.lang.to_string())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<T>().await {
                Ok(result) => Ok(result),
                Err(_) => Err(Gw2ApiError::new(
                    "The response didn't match the expected shape.",
                )),
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(Gw2ApiError::new("Unauthorized.")),
            _ => Err(Gw2ApiError::new("Something unexpected happened.")),
        }
    }
}

#[derive(Debug)]
pub struct Gw2ApiError {
    pub error: String,
}

impl Gw2ApiError {
    pub fn new(msg: &str) -> Gw2ApiError {
        Self {
            error: msg.to_string(),
        }
    }
}

impl From<reqwest::Error> for Gw2ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            error: format!("{:?}", err),
        }
    }
}

