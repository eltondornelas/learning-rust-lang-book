/* Test Organization (integration test and unit test)
***************************************************************************************/
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}


/* writing and controlling tests
**************************************************************************************/
// #[derive(PartialEq, Debug)]
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }
//
// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }
//
// pub fn greeting_for_error(name: &str) -> String {
//     String::from("Hello!")
// }
//
//
// pub struct Guess {
//     value: i32,
// }
//
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//
//         Guess { value }
//     }
// }
//
//
// #[cfg(test)]  // unit test
// mod tests {
//     use super::*;
//
//     //----------------------------------------------------------------//
//     // assert_eq!
//
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
//
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }
//
//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
//
//     #[test]
//     fn it_adds_two_bug() {
//         assert_eq!(3, add_two(2));
//     }
//
//     //---------------------------------------------------//
//     // panic!
//
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
//
//     //------------------------------------------------//
//     // assert!
//
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(larger.can_hold(&smaller));
//     }
//
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(!smaller.can_hold(&larger));  // attention on the negate "!"
//     }
//
//     //-------------------------------------------------------------------------------//
//     // adding custom failures messages
//
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(result.contains("Carol"));
//     }
//
//     #[test]
//     fn greeting_contains_name_custom_message() {
//         let result = greeting_for_error("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`",
//             result
//         );
//     }
//
//
//     //-----------------------------------------------------------------------------------//
//     // should_panic
//
//     #[test]
//     // #[should_panic]
//     #[should_panic(expected = "Guess value must be between 1 and 100, got ")]
//     fn greater_than_100() {
//         Guess::new(100);
//     }
//
//
//     //---------------------------------------------------------------------------------//
//     // using Result<T, E> in test
//
//     #[test]
//     fn it_works_with_result() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
//
//     // ignore
//     #[test]
//     #[ignore]
//     fn expensive_test() {
//         println!("this test has been ignored!!!")
//     }
// }

// cargo test --help
// cargo test -- --help
// cargo test -- --test-threads=1  // running tests consecutively
// cargo test -- --show-output  // shows the println! of functions
// cargo test it_works  // running a single test
// cargo test -- --ignored  // running ignored tests
// cargo test --test integration_test