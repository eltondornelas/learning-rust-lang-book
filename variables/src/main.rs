fn main() {
    const MAX_POINTS: u32 = 100_000;
    // underscores can be inserted in numeric literals to improve readability

    // let x = 5;  // immutable variable by default
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // difference in shadowing and mut
    //// shadowing can reuse the same name with different type of the value
    let spaces = "   ";
    let spaces = spaces.len();

    // mut
    let mut spaces = "   ";
    // spaces = spaces.len();  // type error

    // Data Types
    //// In cases when many types are possible, such as when we converted a String to a numeric type
    //// using parse we must add a type annotation, like this:
    let guess: u32 = "42".parse().expect("Not a number");

    // let guess = "42".parse().expect("Not a number");  // error

    // Integer Overflow
    // let teste: u8 = 256;
    // println!("test {}", teste);

    // Floating-Point Types
    let x = 2.0;
    let y: f32 = 15.3;

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Boolean -> 1 byte
    let t = true;
    let f: bool = false;

    // Character Type -> char = 4 bytes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}",heart_eyed_cat);

    // Compound types
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // tuple direct access
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array Type -> Arrays in Rust have a fixed length, like tuples
    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];  // = let b = [3, 3, 3, 3, 3];

    let first = a[1];
    let second = a[2];

}