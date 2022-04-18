use serde::{Deserialize, Serialize};

use reqwest::Client;

pub const API_URL: &str = "https://finiam-secrets.herokuapp.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoom {
    expiry: u32,
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    room_id: String,
}

pub struct APIClient {
    client: Client,
}

impl APIClient {
    pub fn new() -> Self {
        APIClient {
            client: Client::new(),
        }
    }

    pub async fn create_secret(
        &self,
        secret: String,
        expiry: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = CreateRoom {
            expiry: expiry,
            secret: secret,
        };

        let url = format!("{}/api/secrets", API_URL);

        let room: Room = self
            .client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(room.room_id)
    }

    //pub async fn check_if_room_exists(&self, room: &str) -> bool {}

    //pub async fn get_room_secret(&self, room: &str) -> String {}

    //pub async fn delete_secret(&self, room: &str) {}

    //pub async fn get_stats(&self) -> u32 {}
}
