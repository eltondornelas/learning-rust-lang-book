pub fn patterns_example_main() {
    // conditional_if_let_else_expressions();
    // while_let_conditional_loop();
    // for_loop();
    // let_statements();
    function_parameters();
}

fn function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn let_statements() {
    let x = 5; // let PATTERN = EXPRESSION

    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3);  // error
    let (x, y, _) = (1, 2, 3);
}

fn for_loop() {
    // at for x in y the x is the pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn while_let_conditional_loop() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn conditional_if_let_else_expressions() {
    let favorite_color: Option<&str> = None;
    // let favorite_color: Option<&str> = Some("blue");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}