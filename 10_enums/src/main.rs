/* if let
 **************************************************************************************************/

use crate::Coin::Quarter;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let some_u8_value = Some(3u8);
    let some_u8_value = Some(0u8);

    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alaska);

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}




// /* _ placeholder
//  **************************************************************************************************/
//
// fn main() {
//     let some_u8_value = 0u8;
//     // let some_u8_value: u8 = 0;  // same as above?
//
//     match some_u8_value {
//         1 => println!("one"),
//         3 => println!("three"),
//         5 => println!("five"),
//         7 => println!("seven"),
//         _ => (),
//     }
//
//     // println!("{}", some_u8_value)
// }



// /* match with Option<T>
//  **************************************************************************************************/
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
//
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//
//     println!("{:#?}", six);
//     println!("{:#?}", none);
// }



// /* Patterns that Bind to Values
//  **************************************************************************************************/
//
// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
//
// fn main() {
//     let coin = Coin::Quarter(UsState::Alaska);
//     let ret = value_in_cents(coin);
//
//     println!("{}", ret);
// }



// /* The coin example
//  **************************************************************************************************/
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
//
// fn main() {
//     let coin = Coin::Penny;
//     let ret = value_in_cents(coin);
//
//     println!("{}", ret);
// }



// /* Option<T>
//  **************************************************************************************************/
//
// // this is just to show the Option<T> from the standard library of Rust
// // https://doc.rust-lang.org/std/option/enum.Option.html
// // enum Option<T> {
// //     Some(T),
// //     None,
// // }
//
// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");
//
//     let absent_number: Option<i32> = None;
//     // let absent_number: Option<i32> = Option::None;
//
//
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//     // let y: Option<i8> = Option::Some(5);
//
//     // let sum = x + y;  // compile error
//
// }


// /* the example before with match
//  **************************************************************************************************/
//
// #[derive(Debug)]
// enum Message {
//     Quit,
//     // Move { x: i32, y: i32 },
//     Write(String),
//     // ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         println!("{:#?}", self);
//         // there are other ways of getting only the value of the variant of enum
//     }
//
//     fn value(&self) {
//         match self {
//             Message::Write(string) => {
//                 println!("{}", string)
//             },
//             Message::Quit => println!("ITS QUIT")
//         }
//     }
//
//     fn value_without_self(message: Message) {
//         match message {
//             Message::Write(string) => {
//                 println!("{}", string)
//             },
//             Message::Quit => println!("ITS QUIT")
//         }
//     }
// }
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     // m.call();
//     m.value();
//     Message::value_without_self(m);  // as associated function
// }


// /* new example with multiple types
//  **************************************************************************************************/
//
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         println!("{:#?}", self);
//         // there are other ways of getting only the value of the variant of enum
//     }
// }
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }





// /* putting data directly into enum
//  **************************************************************************************************/
// // enum IpAddr {
// //     V4(String),
// //     V6(String),
// // }
//
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// fn main() {
//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     // let loopback = IpAddr::V6(String::from("::1"));
//
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }



/* using struct to represent
 **************************************************************************************************/
//
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//
//
// }
