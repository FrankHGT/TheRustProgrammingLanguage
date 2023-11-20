use std::fmt::Display;

use struct_lifetime::ImportantExcerpt;

fn dangling_reference() {
        // let r;                // ---------+-- 'a
        //                       //          |
        // {                     //          |
        //     let x = 5;        // -+-- 'b  |
        //     r = &x;           //  |       |
        // }                     // -+       |
        //                       //          |
        // println!("r: {}", r); //          |


    let x = 5;         // ----------+-- 'b
                            //           |
    let r = &x;       // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // --+       |
}                           // ----------+

fn lifetime_test() {
    let string1 = String::from("abcd");
    let string2 = "xyzxxx";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // lifetime(result) = min(lifetime(string1), lifetime(string2)) = lifetime(string2)
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), &string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// only take x referencea, so we doesn't care about y's lifetime
fn _longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// if we return a reference, it must be associated with input parameter
// fn _longest3<'a>(_x: &str, _y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

mod struct_lifetime;

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };

    // method lifetime
    println!("level of i: {}", i.level());

    // the thrid elision rule
    println!("{}", i.announce_and_return_part("didudidu"));
}

fn lifetime_elision() {
    // same as fn first_word<'a>(s: &'a str) -> &'a str
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }

    first_word("ffff");
}

fn static_lifetime() {
    // static lifetime means reference is availble in whole time of execution
    let s: &'static str = "I have a static lifetime.";

    println!("{}", s);
}

fn combine_trait_parameter_constrains_and_lifetime() {
    let s1 = String::from("frank");
    let s2 = "hgt";

    let s = longest_with_an_announcement(s1.as_str(), s2, "didudidu");

    println!("who is longer? : {}", s);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
println!("Announcement! {}", ann);

if x.len() > y.len() {
    x
} else {
    y
}
}

fn main() {
    dangling_reference();
    lifetime_test();
    struct_lifetime();
    lifetime_elision();
    static_lifetime();
    combine_trait_parameter_constrains_and_lifetime();
}
