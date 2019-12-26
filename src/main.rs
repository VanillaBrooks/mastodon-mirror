use mastodon_mirror;
use mastodon_mirror::config;
fn main() {
    let res = config::read_config(None);
    dbg! {&res};
    let mirror = config::init_mirrors(res.unwrap());
    dbg! {&mirror};
}
