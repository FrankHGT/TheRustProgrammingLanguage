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

    let some_summarizable = returns_summarizable();
    println!("some summarizable: {}", some_summarizable.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"), 
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// can't return different type, even they all impl Summary
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }