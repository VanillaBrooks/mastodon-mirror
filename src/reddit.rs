use super::error;
use bytes;
use gfycat;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct RedditData {
    data: DataInfo,
}
#[derive(Debug, Deserialize)]
struct DataInfo {
    children: Vec<PostWrapper>,
}

#[derive(Debug, Deserialize)]
struct PostWrapper {
    data: Post,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    title: String,
    link_flair_css_class: Option<String>,
    is_self: bool,
    #[serde(rename(deserialize = "ups"))]
    upvotes: u32,
    #[serde(rename(deserialize = "downs"))]
    downvotes: u32,
    created: f64,
    id: String,
    url: String,
    domain: String,
}

impl std::cmp::PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Post {
    pub fn is_valid(&self) -> bool {
        if self.is_self {
            return false;
        }

        if self.domain == "i.redd.it"
            || self.domain.contains("i.imgur")
            || self.domain.contains("gfycat.com")
        {
            return true;
        }

        false
    }
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn id_own(self) -> String {
        self.id
    }
    pub fn upvotes(&self) -> &u32 {
        &self.upvotes
    }
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Download a reddit post
    pub async fn download(
        self,
        client: &reqwest::Client,
        gc: &gfycat::Api,
    ) -> Result<bytes::Bytes, error::Reddit> {
        let url: String = if self.url.contains("gfycat") {
            let stripped = strip_gfycat(&self.url)?;

            gc.info(stripped).await?.webm_url
        } else {
            self.url
        };

        let response = client.get(&url).send().await?.bytes().await?;
        Ok(response)
    }
}

pub async fn get_posts(
    client: &reqwest::Client,
    subreddit: &str,
) -> Result<Vec<Post>, error::Reddit> {
    let res = client.get(subreddit).send().await?.bytes().await?;

    let data: RedditData = serde_json::from_slice(&res)?;

    let posts = data.data.children.into_iter().map(|x| x.data).collect();
    Ok(posts)
}

fn strip_gfycat(input: &str) -> Result<&str, error::Reddit> {
    // checks if the first character is a  "/" and removes it if it is
    let input = if let Some(last_character) = input.chars().last() {
        if last_character == '/' {
            &input[0..input.len() - 1]
        } else {
            input
        }
    } else {
        return Err(error::Reddit::ParseGfycat);
    };

    // split the url into the ending segment
    let chunks = if let Some(chunk) = input.split('/').last() {
        chunk
    } else {
        return Err(error::Reddit::ParseGfycat);
    };

    // grab the first element in a hyphen-separated-list
    let first_hypen = if let Some(first) = chunks.split('-').next() {
        first
    } else {
        return Err(error::Reddit::ParseGfycat);
    };

    Ok(first_hypen)
}

/// Test extrapolation of what the webm url would be for a given base gfycat url
/// As it turns out this method is not currently accurate :(
fn gfycat_webm(base_url: &str) -> String {
    "https://giant.gfycat.com/".to_owned() + base_url + ".webm"
}

pub async fn test_webm_extrap(full_url: &str, gc: &gfycat::Api) -> (String, String) {
    let strip = strip_gfycat(full_url).unwrap();
    let gc_url = gc.info(strip).await.unwrap().webm_url;
    let extrap = gfycat_webm(strip);
    (extrap.to_ascii_lowercase(), gc_url.to_ascii_lowercase())
}

#[test]
fn test_strip_1() {
    let url = "https://gfycat.com/babyishsoftleveret-babushka-flowers-waiting-welcome-hello";
    let right = "babyishsoftleveret";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_2() {
    let url = "https://gfycat.com/obviousoldgrayreefshark-jennifer-carpenter-yvonne-strahovski";
    let right = "obviousoldgrayreefshark";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_3() {
    let url = "https://gfycat.com/arcticunhealthylacewing";
    let right = "arcticunhealthylacewing";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_4() {
    let url = "https://gfycat.com/agitatedplaintiveatlasmoth-aimee-garcia";
    let right = "agitatedplaintiveatlasmoth";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_5() {
    let url = "https://gfycat.com/ashamedhugeharpyeagle-happy-new-year-new-years-holiday";
    let right = "ashamedhugeharpyeagle";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_7() {
    let url = "https://gfycat.com/glitteringdeafeningindianskimmer-happy-new-year-new-years-holiday-hanson";
    let right = "glitteringdeafeningindianskimmer";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_8() {
    let url = "https://gfycat.com/meekwiltedatlanticbluetang-happy-new-year-new-years-holiday/";
    let right = "meekwiltedatlanticbluetang";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_9() {
    let url = "https://gfycat.com/sinfularidfluke/";
    let right = "sinfularidfluke";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_10() {
    let url = "https://gfycat.com/brilliantessentialdouglasfirbarkbeetle";
    let right = "brilliantessentialdouglasfirbarkbeetle";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}
#[test]
fn test_strip_11() {
    let url = "https://gfycat.com/delayedimmaculatefrigatebird-football";
    let right = "delayedimmaculatefrigatebird";

    let left = strip_gfycat(url);
    dbg! {&left};
    assert_eq! {left.unwrap(), right};
}

//
// the following blocks check to ensure the webm extrapolation method is correct (its not)
//
#[allow(dead_code)]
async fn init_gfycat() -> gfycat::Api {
    let creds = gfycat::LoadCredentials::new(std::path::Path::new("gfycat.json")).unwrap();
    gfycat::Api::from_credentials(&creds).await.unwrap()
}

#[tokio::test]
async fn webm_1() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/babyishsoftleveret-babushka-flowers-waiting-welcome-hello";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_2() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/obviousoldgrayreefshark-jennifer-carpenter-yvonne-strahovski";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_3() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/arcticunhealthylacewing";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_4() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/agitatedplaintiveatlasmoth-aimee-garcia";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_5() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/glitteringdeafeningindianskimmer-happy-new-year-new-years-holiday-hanson";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_6() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/sinfularidfluke/";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_7() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/brilliantessentialdouglasfirbarkbeetle";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_8() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/delayedimmaculatefrigatebird-football2";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
#[tokio::test]
async fn webm_9() {
    let gc = init_gfycat().await;

    let url = "https://gfycat.com/brilliantessentialdouglasfirbarkbeetle";
    let (l, r) = test_webm_extrap(url, &gc).await;
    assert_eq! {l,r};
}
