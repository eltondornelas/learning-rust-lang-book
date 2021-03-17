fn main() {

    // let test: String = String::from("teste string");
    // let ret = first_word(&test);
    // let ret2 = first_word_with_slice(&test);
    //
    // println!("Return 1: {}", ret);
    // println!("Return 2: {}", ret2);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_with_slice_str(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_with_slice_str(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_with_slice_str(my_string_literal);

    // -------------------------------------------------------------------------------- //
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    // println!("{}", std::convert::From::from(&slice));

    for (i, &item) in slice.iter().enumerate() {
        println!("Index: {} and Item: {}", i, &item);
    }

}

fn first_word_with_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn wrong() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
