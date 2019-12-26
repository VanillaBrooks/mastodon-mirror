use serde_derive::{Deserialize, Serialize};
use serde_yaml;

use elefren;
use std::time;
use std::collections::HashSet;

use super::error;

pub type AllSync = Vec<Sync>;
pub type AllMirror = Vec<Mirror>;

type Token = elefren::Mastodon;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sync {
    #[serde(rename = "reddit")]
    subreddit_ext: String,
    #[serde(rename = "frequency")]
    update_interval: u64,
    client_id: String,
    client_secret: String,
    base: String,
    #[serde(default = "def_path")]
    redirect: String,
    token: String,
}

fn def_path() -> String {
    "https://github.com/VanillaBrooks/mastodon-mirror".into()
}

#[derive(Debug)]
pub struct Mirror {
    pub subreddit_url: String,
    pub token: Token,
    next_update: time::Duration,
    previous_ids: HashSet<String>,
    ids_to_post: Vec<String>,
}
impl Mirror {
    fn new(sync: Sync) -> Result<Self, error::Config> {
        // TODO: do mastodon auth
        let data = elefren::Data {
            base: sync.base.into(),
            client_id: sync.client_id.into(),
            client_secret: sync.client_secret.into(),
            redirect: sync.redirect.into(),
            token: sync.token.into(),
        };

        let token = elefren::Mastodon::from(data);

        Ok(Self {
            next_update: std::time::Duration::from_secs(sync.update_interval),
            subreddit_url: "https://reddit.com/".to_string() + &sync.subreddit_ext,
            token,
            previous_ids: HashSet::new(),
            ids_to_post: vec![]
        })
    }

    pub fn find_unposted_ids(&self, input_ids: Vec<String>) -> Vec<String> {
        input_ids.into_iter()
            .filter(|x| !self.previous_ids.contains(x) && !self.ids_to_post.contains(&x))
            .collect()
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
