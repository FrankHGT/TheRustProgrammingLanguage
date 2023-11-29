#[derive(Debug)]
enum Status {
    Value(u32),
    _Stop,
}

pub fn run() {
    let list_of_numbers = vec![1, 2, 3];

    let _list_of_strings: Vec<String> = list_of_numbers
                                        .iter()
                                        .map(|i| i.to_string())
                                        .collect();

    // function pointer can fit in all kinds of closure,
    // cause fn implemented Fn, FnMut, FnOnce
    let _list_of_strings: Vec<String> = list_of_numbers
                                        .iter()
                                        .map(ToString::to_string)
                                        .collect();


    let list_of_statuses: Vec<Status> = (0u32..20)
                                        .map(Status::Value)
                                        .collect();

    println!("{:?}", list_of_statuses);
}