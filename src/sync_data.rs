use super::{config, error, reddit};
use std::time::{Instant, Duration};
use reqwest;

pub fn cycle(mirrors: &config::AllMirror, client: &reqwest::Client) -> Result<(), error::SyncData> {
    let last_update : Instant= Instant::now() - Duration::from_secs(10);

    for mir in mirrors {
        let posts = reddit::get_posts(&client, &mir.subreddit_url);
        
    }



    unimplemented!{}
}