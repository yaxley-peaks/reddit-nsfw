use roux::Subreddit;
fn main() {
    let sub = Subreddit::new("196");
    let hot = sub.hot(25, None).unwrap();

    hot.data.children.iter().for_each(|thing| {
        if thing.data.over_18 {
            println!("https://www.reddit.com/{}", thing.data.permalink)
        }
    });
}
