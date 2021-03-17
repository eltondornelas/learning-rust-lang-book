/* Separating Modules into Different Files
 * example with front_of_house as a directory
**************************************************************************************************/

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}



// /* Separating Modules into Different Files
//  * example with front_of_house on crate source
// **************************************************************************************************/
//
// mod front_of_house;
//
// pub use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }






// /* example with use keyword
// **************************************************************************************************/
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// use crate::front_of_house::hosting;  // idiomatic way
// // use self::front_of_house::hosting;  // use keyword as relative path
// use crate::front_of_house::hosting::add_to_waitlist;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();  // idiomatic way
//     add_to_waitlist();
//
// }




// /*
// **************************************************************************************************/
//
// // In the restaurant industry, some parts of a restaurant are referred to as front of house and others
// // as back of house. Front of house is where customers are; this is where hosts seat customers,
// // servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and
// // cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
//
//
//
// // example with Relative Paths with super
// fn serve_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
//
//     fn cook_order() {}
// }
//
//
//
//
// // Making Structs and Enums Public
// mod back_of_house_2 {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant_2() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house_2::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }
//
//
//
// // enum example with pub (no need to put pub in the variants
// mod back_of_house_3 {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// pub fn eat_at_restaurant_3() {
//     let order1 = back_of_house_3::Appetizer::Soup;
//     let order2 = back_of_house_3::Appetizer::Salad;
// }