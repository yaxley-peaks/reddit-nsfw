use crate::args::SortBy;
use roux::{util::FeedOption, Subreddit};
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
            //TODO: Fix this later
            let mut after = dat.data.after.unwrap();
            loop {
                if res.len() >= lim as usize {
                    break;
                }
                println!("got {} posts", res.len());
                let _a_opt = FeedOption::new().after(&after);

                let dat = sub.top(lim, Some(_a_opt)).unwrap();

                dat.data.children.iter().for_each(|thing| {
                    if thing.data.over_18 {
                        res.push(thing.data.permalink.clone());
                    }
                });
                after = dat.data.after.unwrap();
            }
        }
        SortBy::Hot => {
            let dat = sub.hot(lim, None).unwrap();

            dat.data.children.iter().for_each(|thing| {
                if thing.data.over_18 {
                    res.push(thing.data.permalink.clone());
                }
            });
            //TODO: Fix this later
            let mut after = dat.data.after.unwrap();
            loop {
                if res.len() >= lim as usize {
                    break;
                }
                println!("got {} posts", res.len());
                let _a_opt = FeedOption::new().after(&after);

                let dat = sub.hot(lim, Some(_a_opt)).unwrap();

                dat.data.children.iter().for_each(|thing| {
                    if thing.data.over_18 {
                        res.push(thing.data.permalink.clone());
                    }
                });
                after = dat.data.after.unwrap();
            }
        }
        SortBy::Rising => {
            let dat = sub.rising(lim, None).unwrap();

            dat.data.children.iter().for_each(|thing| {
                if thing.data.over_18 {
                    res.push(thing.data.permalink.clone());
                }
            });
            //TODO: Fix this later
            let mut after = dat.data.after.unwrap();
            loop {
                if res.len() >= lim as usize {
                    break;
                }
                println!("got {} posts", res.len());
                let _a_opt = FeedOption::new().after(&after);

                let dat = sub.rising(lim, Some(_a_opt)).unwrap();

                dat.data.children.iter().for_each(|thing| {
                    if thing.data.over_18 {
                        res.push(thing.data.permalink.clone());
                    }
                });
                after = dat.data.after.unwrap();
            }
        }
        SortBy::New => {
            let dat = sub.latest(lim, None).unwrap();

            dat.data.children.iter().for_each(|thing| {
                if thing.data.over_18 {
                    res.push(thing.data.permalink.clone());
                }
            });
            //TODO: Fix this later
            let mut after = dat.data.after.unwrap();
            loop {
                if res.len() >= lim as usize {
                    break;
                }
                println!("got {} posts", res.len());
                let _a_opt = FeedOption::new().after(&after);

                let dat = sub.latest(lim, Some(_a_opt)).unwrap();

                dat.data.children.iter().for_each(|thing| {
                    if thing.data.over_18 {
                        res.push(thing.data.permalink.clone());
                    }
                });
                after = dat.data.after.unwrap();
            }
        }
    }
    res
}
