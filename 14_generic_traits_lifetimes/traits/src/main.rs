/* default impl calling another method
**************************************************************************/
use std::borrow::Borrow;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {  // trait as parameter
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {  // trait as parameter with trait bound syntax (syntax sugar)
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {  // returning a type that implements a Trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // println!("1 new tweet: {}", tweet.summarize_author());

    notify(&tweet);  // trait as parameter
}




/*  example of trait and a default impl
*************************************************************************************/
// pub trait Summary {
//     // fn summarize(&self) -> String;  // without having a default
//     fn summarize(&self) -> String {  // including a default impl
//         String::from("(Read more...)")
//     }
// }
//
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
//
// impl Summary for NewsArticle {}  // default impl for NewsArticle
//
// // impl Summary for NewsArticle {
// //     fn summarize(&self) -> String {
// //         format!("{}, by {} ({})", self.headline, self.author, self.location)
// //     }
// // }
//
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
//
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }
//
// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };
//
//     println!("1 new tweet: {}", tweet.summarize());
//
//     //--------------------------------------------------------------------------------------//
//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };
//
//     println!("New article available! {}", article.summarize());
//
// }
