use serde::{Deserialize, Serialize};

use reqwest::{Client, StatusCode};

pub const API_URL: &str = "https://finiam-secrets.herokuapp.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoom {
    expiry: u32,
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomID {
    room_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSecret {
    has_passphrase: bool,
    secret: String,
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
        let params = CreateRoom { expiry, secret };
        let url = format!("{}/api/secrets", API_URL);

        let room: RoomID = self
            .client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(room.room_id)
    }

    pub async fn check_if_room_exists(
        &self,
        room_id: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!(
            "{url}/api/secrets/{room_id}",
            url = API_URL,
            room_id = room_id
        );

        let response = self.client.head(url).send().await?;

        Ok(response.status() == StatusCode::OK)
    }

    pub async fn get_room_secret(
        &self,
        room_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{url}/api/secrets/{room_id}",
            url = API_URL,
            room_id = room_id
        );

        let response = self.client.get(url).send().await?;
        let room: RoomSecret = response.json().await?;

        Ok(room.secret)
    }

    //pub async fn delete_secret(&self, room: &str) {}

    //pub async fn get_stats(&self) -> u32 {}
}
