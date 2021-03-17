use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        /*
        The :: syntax in the ::new line indicates that new is an associated function of the String type.
         An associated function is implemented on a type, in this case String,
         rather than on a particular instance of a String. Some languages call this a static method.
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
        If we hadn’t put the use std::io line at the beginning of the program,
         we could have written this function call as std::io::stdin

        The & indicates that this argument is a reference, which gives you a way to let multiple parts
         of your code access one piece of data without needing to copy that data into memory multiple times.
        */

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*
        Switching from an expect call to a match expression is how you generally move from crashing
         on an error to handling the error. Remember that parse returns a Result type and Result is
         an enum that has the variants Ok or Err. If parse is able to successfully turn the string
         into a number, it will return an Ok value that contains the resulting number and the
         match expression will just return the num value that parse produced and put inside the Ok value.
         The underscore, _, is a catchall value; in this example, we’re saying we want to match all
         Err values, no matter what information they have inside them.
        */

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
