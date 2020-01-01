use gfycat;
use mastodon_mirror;
use mastodon_mirror::config;
use reqwest;
use tokio;

use bytes;
use std::fs;
use std::io::prelude::*;
async fn download_url(save_name: &str, url: &str) -> Result<(), reqwest::Error> {
    let req = reqwest::get(url).await?.bytes().await?;
    let mut file = fs::File::create(save_name).unwrap();
    file.write_all(&req);
    Ok(())
}

async fn download_bytes(url: &str) -> bytes::Bytes {
    reqwest::get(url).await.unwrap().bytes().await.unwrap()
}
#[tokio::main]
async fn main() {
    let res = config::read_config(None);
    dbg! {&res};
    let mirror = config::init_mirrors(res.unwrap()).await;
    // dbg! {&mirror};

    let mut mirror = mirror.unwrap();

    let b_ = download_bytes("https://i.redd.it/wagmu5qw4e741.jpg").await;

    let x = mirror[0].mastodon_api.attach_picture(b_).await;
    // let x = mirror[0].mastodon_api.post_status("test status").await;
    dbg! {x};

    // let x = mirror[0].mastodon_api.verify_creds().await;
    // dbg! {x};

    // let client = reqwest::Client::new();
    // let res = mastodon_mirror::sync_data::cycle_reddit(&mut mirror, &client);

    // let start_url = "glitteringdeafeningindianskimmer";
    // let creds = gfycat::LoadCredentials::new(std::path::Path::new("gfycat.json")).unwrap();
    // let gc = gfycat::Api::from_credentials(&creds).await.unwrap();
    // let webm = gc.info(start_url).await.unwrap().webm_url;
    // dbg! {&webm};

    // let res = download_url("test.webm", &webm).await;
    // dbg! {&res};

    // check_extrap(&gc).await
}
