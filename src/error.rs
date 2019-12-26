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
}

from! {std::io::Error, Config::OpenFile}
from! {serde_yaml::Error, Config::Serde}
