use roux::Subreddit;
fn main() {
    let sub = Subreddit::new("rust");
    let top = sub.top(10, None).unwrap();
    top.data.children.iter().for_each(|thing| {
        println!("{}", thing.data.permalink);
    });
}