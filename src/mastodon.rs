use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use bytes;

use super::error;
#[derive(Debug, Deserialize)]
struct UserAuth {
    code: String,
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u32,
}

#[derive(Debug)]
pub struct Api {
    client: reqwest::Client,
    expiration: Instant,
    created_at: Instant,
    token: String,
    auth_data: Data,
}

impl Api {
    pub async fn new(data: Data) -> Result<Self, error::Mastodon> {
        let client = reqwest::Client::new();

        let endpoint = data.base.to_owned() + "/oauth/authorize";

        // let json = serde_json::json! {{
        //     "response_type": "code",
        //     "client_id": data.client_id.clone(),
        //     "redirect_uri": data.redirect_uri.clone(),
        //     "scope": data.scope.clone()
        // }};

        // let form = reqwest::multipart::Form::new()
        //     .text("response_type", "code")
        //     .text("client_id", data.client_id.clone())
        //     .text("redirect_url", data.redirect_uri.clone())
        //     .text("scope", data.scope.clone());

        // let form = reqwest::multipart::Form::new()
        //     .text("Username", "_")
        //     .text("Password", "_");

        // let mut user_auth = dbg!{client
        //     .get(&endpoint)
        //     .query(&[
        //         ("response_type", "code"),
        //         ("client_id", &data.client_id),
        //         ("redirect_uri", "urn:ietf:wg:oauth:2.0:oob"),
        //         ("scope", &data.scope),
        //     ])}
        //     .multipart(form)
        //     .send()
        //     .await?;

        // dbg! {&user_auth};
        // dbg!{user_auth.url().query()};
        // dbg!{user_auth.text().await};

        // dbg!{user_auth.text().await};

        let endpoint = data.base.to_owned() + "/oauth/token";

        let mut response = client
            .post(&endpoint)
            .json(&data)
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        // TODO: convert response.created_at to Instant to store here

        Ok(Self {
            client,
            expiration: Instant::now(),
            token: response.access_token,
            auth_data: data,
            created_at: Instant::now(),
        })
    }

    pub async fn attach_picture(&self, data: bytes::Bytes) -> Result<Attachment, error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/media";

        let slice = data.into_iter().collect::<Vec<u8>>();

        let file_part = reqwest::multipart::Part::bytes(slice);

        let file = reqwest::multipart::Form::new().part("file", file_part);

        dbg! {&self.token};
        let token = "Bearer ".to_owned() + &self.token;

        let mut response = dbg! {self
        .client
        .post(&endpoint)
        .bearer_auth(&self.token)
        // .header("Authorization", &token)
        .multipart(file)}
        .send()
        .await?;
        dbg! {&response};

        dbg! {response.text().await};

        Ok(Attachment::default())
    }

    pub async fn post_status(
        &self,
        // attach: &Attachment,
        text: &str,
    ) -> Result<(), error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/statuses";

        let form = reqwest::multipart::Form::new().text("status", text.to_owned());

        let form = serde_json::json! {{"status": text.to_owned()}};

        let mut response = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.token)
            // .header("Authorization", format!{"Bearer {}", &self.token})
            // .multipart(form)}
            .json(&form)
            .send()
            .await?;

        dbg! {&response};
        dbg! {response.text().await};

        Ok(())
    }

    pub async fn verify_creds(&self) -> Result<(), error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/accounts/verify_credentials";

        let response = self
            .client
            .get(&endpoint)
            .bearer_auth(&self.token)
            .send()
            .await?
            .text()
            .await;

        dbg! {response};

        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Attachment {
    id: String,
    #[serde(rename = "type")]
    picture_type: String,
    url: String,
    preview_url: String,
    text_url: String,
    meta: PictureInfo,
    description: String,
    blurhash: String,
}

#[derive(Debug, Deserialize, Default)]
struct PictureInfo {
    Focus: Point,
    original: PictureDimensions,
    small: PictureDimensions,
}

#[derive(Debug, Deserialize, Default)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize, Default)]
struct PictureDimensions {
    width: u32,
    height: u32,
    size: String,
    aspect: f32,
}

#[derive(Deserialize, Default, Debug, Serialize)]
pub struct Data {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scope: String,
    #[serde(default = "init_grant_type")]
    grant_type: String,
    base: String,
    #[serde(skip_serializing)]
    code: String
}

fn init_grant_type() -> String {
    "client_credentials".into()
}
