use super::error;
use serde::Deserialize;
use serde_json;

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

impl Post {
    pub fn is_valid(&self) -> bool {
        if self.is_self == true {
            return false;
        }

        if self.domain == "i.redd.it" || self.domain.contains("i.imgur") {
            return true;
        }

        return false;
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
}

pub fn get_posts(client: &reqwest::Client, subreddit: &str) -> Result<Vec<Post>, error::Reddit> {
    let res = client.get(subreddit).send()?;

    let data: RedditData = serde_json::from_reader(res)?;

    let posts = data.data.children.into_iter().map(|x| x.data).collect();
    Ok(posts)
}
