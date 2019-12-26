use serde;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;

use std::path;
use std::time;

use super::error;

pub type AllSync = Vec<Sync>;
pub type AllMirror = Vec<Mirror>;

type Token = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sync {
    #[serde(rename = "mastodon")]
    mastodon_api: String,
    #[serde(rename = "reddit")]
    subreddit_ext: String,
    #[serde(rename = "frequency")]
    update_interval: u64,
}

pub struct Mirror {
    pub sync: Sync,
    pub token: Token,
    next_update: time::Duration,
}
impl Mirror {
    fn new(sync: Sync) -> Result<Self, error::Config> {
        // TODO: do mastodon auth
        let token = "".into();

        Ok(Self {
            next_update: std::time::Duration::from_secs(sync.update_interval),
            sync,
            token,
        })
    }
}

pub fn read_config(path: Option<&str>) -> Result<AllSync, error::Config> {
    let path = path.unwrap_or("config.yaml");

    let file = std::fs::File::open(path)?;
    let sync_items: Vec<Sync> = serde_yaml::from_reader(file)?;

    Ok(sync_items)
}

pub fn init_mirrors(config_input: AllSync) -> Result<AllMirror, error::Config> {
    let mirrors = config_input
        .into_iter()
        .map(Mirror::new)
        .filter(|x| {
            if let Err(x) = x {
                println! {"there was an error authenticating a mirror"}
                dbg! {x};
                false
            } else {
                true
            }
        })
        .map(|x| x.unwrap())
        .collect();

    Ok(mirrors)
}
