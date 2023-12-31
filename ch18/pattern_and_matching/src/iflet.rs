pub fn run() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(f) = favorite_color {
        println!("Using your favorite color, {}, as background color", f);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age < 30 {
            println!("Using blue as background color");
        } else {
            println!("Using red as background color");
        }
    } else {
        println!("Using black as background color");
    }
}