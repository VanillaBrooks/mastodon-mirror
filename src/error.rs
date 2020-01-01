use reqwest;
use serde_json;
use serde_yaml;

macro_rules! from {
    ($root:path, $destination_enum:ident :: $path_:ident) => {
        impl From<$root> for $destination_enum {
            fn from(e: $root) -> Self {
                $destination_enum::$path_(e)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    Config(Config),
}

#[derive(Debug)]
pub enum Config {
    OpenFile(std::io::Error),
    Serde(serde_yaml::Error),
    Mastodon(Mastodon),
}

from! {std::io::Error, Config::OpenFile}
from! {serde_yaml::Error, Config::Serde}

#[derive(Debug)]
pub enum Reddit {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Gfycat(gfycat::error::ApiError),
    ParseGfycat,
}

from! {reqwest::Error, Reddit::Reqwest}
from! {serde_json::Error, Reddit::Serde}
from! {gfycat::error::ApiError, Reddit::Gfycat}
from! {Mastodon, Config::Mastodon}

#[derive(Debug)]
pub enum Mastodon {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    SerdeUrl(serde_urlencoded::ser::Error),
    FantoSession(fantoccini::error::NewSessionError),
    FantoCmd(fantoccini::error::CmdError),
    Blank,
}

from! {reqwest::Error, Mastodon::Reqwest}
from! {serde_json::Error, Mastodon::Serde}
from! {serde_urlencoded::ser::Error, Mastodon::SerdeUrl}
from! {fantoccini::error::NewSessionError, Mastodon::FantoSession}
from! {fantoccini::error::CmdError, Mastodon::FantoCmd}

#[derive(Debug)]
pub enum SyncData {}
