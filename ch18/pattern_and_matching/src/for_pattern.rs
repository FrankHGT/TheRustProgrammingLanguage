pub fn run() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
}