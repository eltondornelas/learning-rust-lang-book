fn main() {
    let mut s = String::new();
    //---------------------------------------------//
    let data = "initial contents";
    let s = data.to_string();  // equivalent -> let s = String::from("initial contents");
    // String::from and to_string do the same thing, so which you choose is a matter of style.

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    //--------------------------------------------------//

    // appending with push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";  // if String::from("bar"); the line below will not work
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // appending with push
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);


    //----------------------------------------------------------------------------------//
    // + Operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}", s1);  // error
    println!("{}", s3);

    // format! Macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    //------------------------------------------------------------------------------------//
    // indexing
    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");  // danger with .len();
    println!("{}", hello.len());

    // slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // hello is actually 2 bytes per char because of its UTF-8
    // let s = &hello[0..1];  // will panic as invalid index
    println!("{}", s);


    //-----------------------------------------------------------------------------------------//
    // iterating over Strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for (i, b) in "नमस्ते".bytes().enumerate() {
        if i == 17 {
            println!("{}", b);
            break;
        }

        print!("{}, ", b);
    }
}
