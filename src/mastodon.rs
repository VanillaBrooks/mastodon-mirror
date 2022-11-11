use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use fantoccini::{self, Locator};
use serde_urlencoded;

use std::fmt::Write as _;

use bytes;
use reqwest::multipart::Form;

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
        // TODO: convert response.created_at to Instant to store here

        Ok(Self {
            client,
            expiration: Instant::now(),
            // token: response.access_token,
            token: data.code.clone(),
            auth_data: data,
            created_at: Instant::now(),
        })
    }

    // TODO: Check for internal server error here later
    pub async fn attach_picture(&self, data: bytes::Bytes) -> Result<Attachment, error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/media";

        let slice = data.into_iter().collect::<Vec<u8>>();

        let file_part = reqwest::multipart::Part::bytes(slice);

        let file = reqwest::multipart::Form::new().part("file", file_part);

        dbg! {&self.token};

        let mut response = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.token)
            .multipart(file)
            .send()
            .await?;

        dbg!{response.status()};

        dbg! {&response.text().await};

        // let response_json = response.json::<Attachment>().await?;

        // Ok(response_json)
        Ok(Attachment::default())
    }

    pub async fn post_status(
        &self,
        // attach: &Attachment,
        text: &str,
    ) -> Result<(), error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/statuses";

        let form = serde_json::json! {{"status": text}};

        let mut response = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.token)
            .json(&form)
            .send()
            .await?;

        dbg! {&response};
        // dbg! {response.text().await};

        Ok(())
    }

    pub async fn verify_creds(&self) -> Result<(), error::Mastodon> {
        let endpoint = self.auth_data.base.to_owned() + "/api/v1/accounts/verify_credentials";

        let mut response = self
            .client
            .get(&endpoint)
            .bearer_auth(&self.token)
            .send()
            .await?;
            // .json::<AppCredentials>()
            // .await?;
        dbg!{&response.text().await};

        // dbg! {response};

        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct AppCredentials {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    bot: bool,
    created_at: String,
    note: String,
    url: String,
    avatar: String,
    header: String,
    header_static: String,
    followers_count: u32,
    following_count: u32,
    statuses_count: u32,
    last_status_at: Option<String>,
    source: Source,
    fields: Vec<String>,
    emojis: Vec<String>,
}
#[derive(Debug, Deserialize, Default)]
struct Source {
    privacy: String,
    sensitive: bool,
    langugae: Option<String>,
    note: String,
    fields: Vec<String>,
    follow_requests_count: u32,
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
    focus: Point,
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
    #[serde(skip_serializing)]
    base: String,
    #[serde(default)]
    code: String,
    #[serde(skip_serializing)]
    username: String,
    #[serde(skip_serializing)]
    password: String,
}

impl Data{
    fn authorize(&self) -> AuthRequest {
        AuthRequest {
            client_id: &self.client_id,
            redirect_uri: &self.redirect_uri,
            scope: &self.scope,
            response_type: "code"
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthRequest <'a> {
    client_id: &'a str,
    redirect_uri: &'a str,
    scope: &'a str,
    response_type: &'a str
}
impl <'a> AuthRequest <'a>{
    fn url (self, mut base: String) -> Result<String, error::Mastodon> {
        let url = serde_urlencoded::to_string(self)?;
        std::write!(base, "{}", url).unwrap();
        Ok(base)

    }
}

fn init_grant_type() -> String {
    "client_credentials".into()
}
