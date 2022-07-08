use roux::Subreddit;

pub mod args;
pub fn get_posts(sub: &str, _sort: args::SortBy, lim: u32) -> Vec<String> {

    let sub = Subreddit::new(&sub);
    let hot = sub.hot(lim, None).unwrap();
    
    let mut res = Vec::new();
    hot.data.children.iter().for_each(|thing| {
        if thing.data.over_18 {
            res.push(thing.data.permalink.clone());
        }
    });
    res
}
