fn main() {
    // creating vectors
    let v: Vec<i32> = Vec::new();  // create an empty Vector
    let v = vec![1, 2, 3];  // macro to create with values; default i32

    // updating vectors
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading elements
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];  // way 1
    println!("The third element is {}", third);

    match v.get(2) {  // way 2
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100];  // it panics
    let does_not_exist = v.get(100); // it panics

        //  mutable and immutable references in the same scope = error
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    // println!("The first element is: {}", first);  // error

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];  // immutable
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];  // mutabble
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
