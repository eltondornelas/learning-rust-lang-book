// **********************************************************************************************
// creating more methods

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // we don't want ownership
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// multiple impl blocks are possible syntax. Though, there is no need in separate in here
impl Rectangle {
    // this is an Associated Function, cuz it don't take self as parameter
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square rectangle: {:#?}", Rectangle::square(5));
}


// ************************************************************************************************
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         // & cuz we don't want take ownership
//         self.width * self.height
//     }
// }
//
// // defining method syntax
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }


// ***********************************************************************************************
// #[derive(Debug)]  // this way makes possible to print debug
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // Refactoring with structs
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//
//     // println!("rect is {}", rect1);  // error
//     println!("rect is {:?}", rect1);  // {:?} or {:#?} for pretty-print
//     println!("rect is {:#?}", rect1);
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//*************************************************************************************************
// fn main() {
//     // refactoring with tuple to be more readable, making sure that both are necessary
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//*************************************************************************************************
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }