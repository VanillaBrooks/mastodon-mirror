use gfycat;
use mastodon_mirror;
use mastodon_mirror::config;
use reqwest;
use tokio;

use std::fs;
use std::io::prelude::*;
async fn download_url(save_name: &str, url: &str) -> Result<(), reqwest::Error> {
    let req = reqwest::get(url).await?.bytes().await?;
    let mut file = fs::File::create(save_name).unwrap();
    file.write_all(&req);
    Ok(())
}

#[tokio::main]
async fn main() {
    // let res = config::read_config(None);
    // dbg! {&res};
    // let mirror = config::init_mirrors(res.unwrap());
    // dbg! {&mirror};

    // let mut mirror = mirror.unwrap();

    // let client = reqwest::Client::new();
    // let res = mastodon_mirror::sync_data::cycle_reddit(&mut mirror, &client);

    let start_url = "glitteringdeafeningindianskimmer";
    let creds = gfycat::LoadCredentials::new(std::path::Path::new("gfycat.json")).unwrap();
    let gc = gfycat::Api::from_credentials(&creds).await.unwrap();
    let webm = gc.info(start_url).await.unwrap().webm_url;
    dbg! {&webm};

    let res = download_url("test.webm", &webm).await;
    dbg! {&res};

    // check_extrap(&gc).await
}
