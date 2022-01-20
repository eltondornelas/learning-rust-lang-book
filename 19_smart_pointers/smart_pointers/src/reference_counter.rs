use std::cell::RefCell;
use std::rc::Rc;

use crate::reference_counter::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    // Cons(i32, Rc<List>),
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn rc_main() {
    //--------------- start: without Rc<T> ---------------------------//
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    //--------------- end: without Rc<T> ---------------------------//

    //--------------- start: with Rc<T> ---------------------------//
    // // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // // let b = Cons(3, Rc::clone(&a));  // let b = Cons(3, a.clone());  works too but Rust convention uses RC::clone
    // // let c = Cons(4, Rc::clone(&a));
    //
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    //--------------- end: with Rc<T> ---------------------------//

    //--------------- start: with RefCell<T> ---------------------------//
    // now can modify the value stored
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    //--------------- end: with RefCell<T> ---------------------------//

}

