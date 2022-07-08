pub mod args;
use args::SortBy;
use clap::Parser;
use reddit_nsfw::args::SortBy as OSB;
use reddit_nsfw::get_posts;

/**
   Shitty rust thing very shit.
   Why do i have to do this
*/
fn handle_sort_enum(s: SortBy) -> OSB {
    match s {
        SortBy::Top => OSB::Top,
        SortBy::Hot => OSB::Hot,
        SortBy::Rising => OSB::Rising,
        SortBy::New => OSB::New,
    }
}

fn main() {
    let args = args::Opts::parse();
    let sort = handle_sort_enum(args.sort);
    let res = get_posts(&args.sub, sort, 25);
    res.iter().for_each(|x| {
        println!("https://www.reddit.com/{x}");
    });
}
