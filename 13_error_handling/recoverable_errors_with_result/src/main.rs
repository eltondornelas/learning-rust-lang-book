/* Propagating Errors with ? operator
****************************************************************************************/
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
// use std::io;
// use std::io::Read;

fn main() {

}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);

    //----------------------------------------------------------------//
    // shorter way
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

    //-----------------------------------------------------------------//
    // even shorter way
    // fs::read_to_string("hello.txt")
}

// custom validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}



// /* Propagating Errors
// ****************************************************************************************/
// use std::fs::File;
// use std::io::{self, ErrorKind, Read};
// // use std::io;
// // use std::io::Read;
//
// fn main() {
//
// }
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     s = "username".to_string();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }




/* examples with match, unwrap and expect
*********************************************************************************************/
// use std::fs::File;
// use std::io::ErrorKind;
//
// fn main() {
//     let f = File::open("hello.txt");
//
//     // let f: u32 = File::open("hello.txt");  // error. it's just to know what it returns
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
//
//     //-------------------------------------------------------------------------------------//
//     // matching different errors
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         },
//     };
//
//
//     //------------------------------------------------------------------------------------//
//     // using closures to avoid too many match
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
//
//
//     //-----------------------------------------------------------------------------------//
//     // using unwrap or expect
//     let f = File::open("hello.txt").unwrap();
//
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
//     // with expect we can specify the error msg
//
// }