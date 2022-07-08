use crate::args::SortBy;
use roux::Subreddit;
pub mod args;

pub fn get_posts(sub: &str, _sort: SortBy, lim: u32) -> Vec<String> {
    let sub = Subreddit::new(&sub);
    let mut res = Vec::new();
    match _sort {
        SortBy::Top => {
            let dat = sub.top(lim, None).unwrap();
            dat.data.children.iter().for_each(|thing| {
                if thing.data.over_18 {
                    res.push(thing.data.permalink.clone());
                }
            });
        }
        SortBy::Hot => {
            let dat = sub.hot(lim, None).unwrap();
    dat.data.children.iter().for_each(|thing| {
        if thing.data.over_18 {
            res.push(thing.data.permalink.clone());
        }
    });
        },
        SortBy::Rising => {
            let dat = sub.rising(lim, None).unwrap();
    dat.data.children.iter().for_each(|thing| {
        if thing.data.over_18 {
            res.push(thing.data.permalink.clone());
        }
    });
        },
        SortBy::New => {
            let dat = sub.latest(lim, None).unwrap();
    dat.data.children.iter().for_each(|thing| {
        if thing.data.over_18 {
            res.push(thing.data.permalink.clone());
        }
    });
        },
    }

    res
}
