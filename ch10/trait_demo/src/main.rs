use trait_demo::{Tweet, Summary, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"), 
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Beijing"),
        author: String::from("Frank"),
        content: String::from("blabla"),
    };

    println!("default trait impl for NewsArticle: {}", news.summarize());
}