pub fn advanced_functions_and_closures_main() {
    function_pointers();
    returning_closures();
}

fn returning_closures() {
    // won't compile
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

fn function_pointers() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    // fn main() {}
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();  // or below
        // list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}