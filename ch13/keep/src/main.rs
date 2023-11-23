use std::{thread, time::Duration, collections::HashMap, hash::Hash};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    capture_value();
    move_keyword();
}

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
    U: Eq + PartialEq + Hash + Clone + Copy,
    V: Clone + Copy,
{
    calculation: T,
    value_map: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
    U: Eq + PartialEq + Hash + Clone + Copy,
    V: Clone + Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value_map.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg.clone(), v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cacher.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cacher.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cacher.value(intensity)
            );
        }
    }
}

fn capture_value() {
    let x = 4;

    let equal_to_x = |z| z == x;
    // can't capture dynamic environment in a fn item
    // fn equal_to_x(z: i32) -> bool { z == x }

    // x = 5;

    // println!("x is rewritten to {}", x);

    let y = 4;

    assert!(equal_to_x(y));
}

fn move_keyword() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

#[test]
fn cacher_and_hold_other_type() {
    let mut cacher = Cacher::new(|value| {
        println!("value: {:?}", value);
        thread::sleep(Duration::from_secs(1));
        value
    });

    let res = cacher.value("didudidu");
    println!("res: {:?}", res);
}