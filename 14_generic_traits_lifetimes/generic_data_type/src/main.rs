/* Generics in Method Definitions - example mixing types
********************************************************************/
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}




// /* Generics in Method Definitions
// ********************************************************************/
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());
//
//     let t = Point {x: 5.5, y: 10.5};
//     println!("{}", t.distance_from_origin());  // p.distance_from_origin() won't work cuz of type i32 and not f32
//
// }


/* Generics in Struct Definition
*******************************************************************/

// // struct Point<T> {  // this way both fields has to be same types
// //     x: T,
// //     y: T,
// // }
//
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let wont_work_on_first_struct_example = Point { x: 5, y: 4.0 };  // doesn't work because needs to be same type
// }








/* Generics in fuction definition
************************************************************************/
// example without the Generic
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//
//     // calling the generic type
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
//
//
// // function with the generic
// // fn largest<T: >(list: &[T]) -> &T {
// // fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {  // impl
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }






//*************************************************************************//
// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }
