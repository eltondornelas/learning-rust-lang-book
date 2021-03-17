fn main() {
    another_function(5);
    another_function_two(10, 20);

    let x = 5;

    // it's a expression (returns a value and don't have ending semicolons "x+1")
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn another_function_two(x: i32, y: i32) {
    println!("The value of x + y is: {}", x + y)
}

fn five() -> i32 {
    5
}