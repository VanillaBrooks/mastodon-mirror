use serde::{Deserialize, Serialize};
use serde_yaml;

use std::collections::HashSet;
use std::time;

use super::error;
use super::reddit;

use super::mastodon;

pub type AllSync = Vec<Sync>;
pub type AllMirror = Vec<Mirror>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sync {
    #[serde(rename = "reddit")]
    subreddit_ext: String,
    #[serde(rename = "frequency")]
    update_interval: u64,
    nsfw: bool,
    tags: Vec<String>,
    mastodon: mastodon::Data,
}

#[derive(Debug)]
pub struct Mirror {
    pub subreddit_url: String,
    pub mastodon_api: mastodon::Api,
    next_update: time::Instant,
    previous_ids: HashSet<String>,
    to_post: Vec<reddit::Post>,
    nsfw: bool,
}
impl Mirror {
    pub async fn new(sync: Sync) -> Result<Self, error::Config> {
        let api = mastodon::Api::new(sync.mastodon).await?;
        let next_update = time::Instant::now()
            .checked_add(time::Duration::from_secs(sync.update_interval))
            .expect("time err");
        let subreddit_url = format!("https://reddit.com/{}", sync.subreddit_ext);

        Ok(Self {
            next_update,
            subreddit_url,
            mastodon_api: api,
            previous_ids: HashSet::new(),
            to_post: vec![],
            nsfw: sync.nsfw,
        })
    }

    // convienence method for filtering out ids that have previously been posted
    pub fn find_unposted_ids(&self, input_ids: Vec<reddit::Post>) -> Vec<reddit::Post> {
        input_ids
            .into_iter()
            .filter(|x| !self.previous_ids.contains(x.id()) && !self.to_post.contains(&x))
            .collect()
    }

    // add new posts into the queue for posting later
    pub fn queue_posts(&mut self, mut new_posts: Vec<reddit::Post>) {
        self.to_post.append(&mut new_posts);
    }

    pub fn can_update(&self) -> bool {
        if self.to_post.len() > 0 {
            true
        } else {
            false
        }
    }

    pub fn post(&self) -> Result<(), error::Config> {
        // TODO: update to mastodon
        unimplemented! {}
    }
}

pub fn read_config(path: Option<&str>) -> Result<AllSync, error::Config> {
    let path = path.unwrap_or("config.yaml");

    let file = std::fs::File::open(path)?;
    let sync_items: Vec<Sync> = serde_yaml::from_reader(file)?;

    Ok(sync_items)
}

pub async fn init_mirrors(config_input: AllSync) -> Result<AllMirror, error::Config> {
    let mut mirrors = Vec::with_capacity(config_input.len());

    for mir in config_input {
        let m = Mirror::new(mir).await;
        if let Ok(data) = m {
            mirrors.push(data);
        } else {
            println! {"there was an error with a client"}
            dbg! {m};
        }
    }

    Ok(mirrors)
}
