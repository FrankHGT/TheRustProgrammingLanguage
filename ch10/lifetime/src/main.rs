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

fn main() {
    dangling_reference();
    lifetime_test();
}
