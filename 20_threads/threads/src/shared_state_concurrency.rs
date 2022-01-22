use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn shared_state_concurrency_main() {
    // mutex_in_single_thread_context();
    mutex_between_multiple_threads();
}

fn mutex_between_multiple_threads() {
    // let counter = Mutex::new(0); // wont compile
    // let counter = Rc::new(Mutex::new(0)); // adding Rc<T> (still wont compile)
    let counter = Arc::new(Mutex::new(0)); // adding Arc<T>
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter); // with Rc<T>
        let counter = Arc::clone(&counter); // with Arc<T>
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex_in_single_thread_context() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}