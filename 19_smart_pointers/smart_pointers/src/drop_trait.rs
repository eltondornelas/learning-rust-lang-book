struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait_main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // start: dropping a value earlier
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    // end: dropping a value earlier

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // println!("CustomSmartPointers created.");
}