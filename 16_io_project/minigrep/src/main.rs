/* splitting code into a Library Crate
******************************************************************/
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}



/* extracting Logic from main with the "run" method
******************************************************************/
// use std::env;
// use std::fs;
// use std::process;
// use std::error::Error;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//         // this way avoid panic
//     });
//
//     println!("Searching for: {}", config.query);
//     println!("In file: {}", config.filename);
//
//     // let contents = fs::read_to_string(config.filename)
//     //     .expect("Something went wrong reading the file");
//
//     // println!("With text:\n{}", contents);
//
//     // run(config);
//     if let Err(e) = run(config) {
//         println!("Application error: {}", e);
//
//         process::exit(1);
//     }
//
//     // $ cargo run the poem.txt
// }
//
// struct Config {
//     query: String,
//     filename: String,
// }
//
// impl Config {
//     fn new(args: &[String]) -> Result<Config, &str> {  // with Result
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//
//         let query = args[1].clone();
//         let filename = args[2].clone();
//
//         Ok(Config { query, filename })
//     }
// }
//
// // fn run(config: Config) {
// //     let contents = fs::read_to_string(config.filename)
// //         .expect("Something went wrong reading the file");
// //     // this can still panic
// //
// //     println!("With text:\n{}", contents);
// // }
//
// fn run(config: Config) -> Result<(), Box<dyn Error>> {  // dyn -> dynamic
//     // to avoid panicking
//     let contents = fs::read_to_string(config.filename)?;  // removed expect for "?"
//
//     println!("With text:\n{}", contents);
//
//     Ok(())
// }



/* creating Config instance and method as constructor with "new"
******************************************************************/
// use std::env;
// use std::fs;
// use std::process;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     // let config = Config::new(&args);
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//         // A nonzero exit status is a convention to signal to the process that called our program
//         // that the program exited with an error state.
//
//         // The process::exit function will stop the program immediately and return the number
//         // that was passed as the exit status code.
//     });
//
//     println!("Searching for: {}", config.query);
//     println!("In file: {}", config.filename);
//
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");
//
//
//     println!("With text:\n{}", contents);
//
//     // $ cargo run the poem.txt
// }
//
// struct Config {
//     query: String,
//     filename: String,
// }
//
// impl Config {
//     // fn new(args: &[String]) -> Config {  // with panic
//     //     if args.len() < 3 {
//     //         panic!("not enough arguments");
//     //     }
//     //
//     //     let query = args[1].clone();
//     //     let filename = args[2].clone();
//     //
//     //     Config { query, filename }
//     // }
//
//     fn new(args: &[String]) -> Result<Config, &str> {  // with Result
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//
//         let query = args[1].clone();
//         let filename = args[2].clone();
//
//         Ok(Config { query, filename })
//     }
// }



// /* refactoring to improve modularity and error handling
// ******************************************************************/
// // "...main.rs handles running the program, and lib.rs handles all the logic of the task at hand."
// use std::env;
// use std::fs;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     // let (query, filename) = parse_config(&args);
//     let config = parse_config(&args);
//
//     // println!("Searching for: {}", query);
//     // println!("In file: {}", filename);
//     println!("Searching for: {}", config.query);
//     println!("In file: {}", config.filename);
//
//
//     // let contents = fs::read_to_string(filename)
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");
//
//     println!("With text:\n{}", contents);
//
//     // $ cargo run the poem.txt
// }
//
// // // returning tuple
// // fn parse_config(args: &[String]) -> (&str, &str) {
// //     let query = &args[1];
// //     let filename = &args[2];
// //
// //     (query, filename)
// // }
//
// struct Config {
//     query: String,
//     filename: String,
// }
//
// // returning struct
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//
//     Config { query, filename }
// }



/* reading a file
******************************************************************/
// use std::env;
// use std::fs;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let program_name = &args[0];
//     let query = &args[1];
//     let filename = &args[2];
//
//     println!("Program name: {}", program_name);
//     println!("Searching for: {}", query);
//     println!("In file: {}", filename);
//
//     let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");
//
//     println!("With text:\n{}", contents);
//
//     // $ cargo run the poem.txt
// }



/* saving argument values in variables
**************************************************************************************/
// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let program_name = &args[0];
//     let query = &args[1];
//     let filename = &args[2];
//
//     println!("Program name: {}", program_name);
//     println!("Searching for {}", query);
//     println!("In file {}", filename);
//
//     // $ cargo run test sample.txt
// }



/* reading argument values
**********************************************************************/
// use std::env;
// // use std::env::args_os;  // if using invalid Unicode
//
// fn main() {
//     let args: Vec<String> = env::args().collect();  // to read command line arguments
//
//     println!("{:?}", args);
//
//     // $ cargo run needle haystack
// }
