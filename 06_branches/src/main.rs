fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    loop_repetition();
    while_repetition();
    for_repetition();
}

fn loop_repetition() {
    let mut counter = 0;

    // loop infinito
    // loop {
    //     println!("again!");
    // }

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_repetition() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_repetition() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // rev -> reverse the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}