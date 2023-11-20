use std::{collections::HashMap, vec};

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Scores: {:#?}", scores);

    hashmap_ownership();

    get_value_from_hashmap();

    iterate_hashmap();

    update_hashmap();
}

fn hashmap_ownership() {
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map: {:#?}", map);

    // String will move in hashmap, so it can't be used again
    // println!("field_name: {}", field_name);
}

fn get_value_from_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_score = scores.get(&String::from("Blue"));
    println!("Blue score: {:?}", blue_score);
}

fn iterate_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (color, score) in &scores {
        println!("{} got {}", color, score);
    }
}

fn update_hashmap() {
    let mut scores = HashMap::new();

    // replace old value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    println!("Scores: {:#?}", scores);

    // insert when key not exist
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(100);

    println!("Scores: {:#?}", scores);

    // update value base on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        // deference mutable reference to change value
        *count += 1;
    }

    println!("map: {:#?}", map);
}
