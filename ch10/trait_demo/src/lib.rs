use core::fmt;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // use default trait method
    // fn summarize(&self) -> String {
    //     format!("{}, by{}({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl trait is just syntactic sugar
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn print_author<T: Summary>(item: &T) {
    println!("Author of news: {}", item.summarize_author());
}

// item1, item2 can be different type, as long as they all impl Summary trait
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Two new: {}, {}", item1.summarize(), item2.summarize());
}

// item1, item2 must be same type
pub fn print_author2<T: Summary>(item1: &T, item2: &T) {
    println!("Author of news 1: {}, news 2: {}", item1.summarize_author(), item2.summarize_author());
}

// multible trait constrain
// no "impl" after "+"
pub fn notify3(item: impl Summary + Display) {
    println!("Display for item: {}", item);
}
pub fn print_author3<T: Summary + Display>(item: T) {
    println!("Test")
}