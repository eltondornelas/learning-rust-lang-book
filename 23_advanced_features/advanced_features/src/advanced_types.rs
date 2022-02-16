pub fn advanced_types_main() {
    // creating_synonym_with_type_aliases();
    // the_never_return_type();
    // dynamically_seized_types_and_sized_trait();
}

fn dynamically_seized_types_and_sized_trait() {
    // doesn't work
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

// fn the_never_return_type() -> ! {
//     // --snip--
//     println!()
// }

fn creating_synonym_with_type_aliases() {
    type Kilometers = i32;  // Kilometers will be treated the same as values of type i32

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
