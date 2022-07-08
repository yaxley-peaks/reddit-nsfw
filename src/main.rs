pub mod args;
use clap::Parser;
use reddit_nsfw::{get_posts, args::SortBy};

fn main() {
    let args = args::Opts::parse();
    let res = get_posts(&args.sub, SortBy::Hot, 25);
    res.iter().for_each(|x| {
        println!("{x}");
    });
}
