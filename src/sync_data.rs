use super::{config, error, reddit};
use reqwest;
use std::time::{Duration, Instant};

const REDDIT_SLEEP_DUR: Duration = Duration::from_secs(2);

/// Pulls information from reddit pages and stores them in the mirror
pub async fn cycle_reddit(
    mirrors: &mut config::AllMirror,
    client: &reqwest::Client,
) -> Result<(), error::SyncData> {
    for mir in mirrors {
        let posts = reddit::get_posts(&client, &mir.subreddit_url).await;

        if let Ok(data) = posts {
            let posts_to_mirror = mir.find_unposted_ids(data);
            mir.queue_posts(posts_to_mirror);
        } else {
            println! {"there was an error pulling reddit data:"}
            dbg! {&posts};
        }

        std::thread::sleep(REDDIT_SLEEP_DUR);
    }

    Ok(())
}

/// mirrors all content from reddit to mastodon
pub fn cylce_mastodon(mirrors: &mut config::AllMirror) -> Result<(), error::SyncData> {
    mirrors
        .into_iter()
        .map(|mut x| _cycle_mastodon_one(&mut x))
        .collect::<Vec<_>>();

    Ok(())
}

/// Recursively post all content
fn _cycle_mastodon_one(mir: &mut config::Mirror) -> Result<(), error::SyncData> {
    if mir.can_update() {
        mir.post();
        Ok(())
    // _cycle_mastodon_one(mir)
    } else {
        Ok(())
    }
}
