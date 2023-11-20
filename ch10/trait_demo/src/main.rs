use std::fmt::{Display, Debug};

use trait_demo::{Tweet, Summary, NewsArticle, notify, print_author, notify3};

fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: T, _u: U) -> i32 {
    5
}

fn _some_function_where<T, U>(_t: T, _u: U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{
    5
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"), 
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    print_author(&tweet);

    let news = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Beijing"),
        author: String::from("Frank"),
        content: String::from("blabla"),
    };

    println!("default trait impl for NewsArticle: {}", news.summarize());
    notify(&news);
    print_author(&news);
    notify3(news);
}