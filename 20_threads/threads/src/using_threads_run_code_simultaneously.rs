use std::thread;
use std::time::Duration;

pub fn using_threads_run_code_simultaneously_main() {
    // creating_new_thread();
    // waiting_for_all_threads_finish();
    using_move_closures_with_threads();
}

fn using_move_closures_with_threads() {
    let v = vec![1, 2, 3];

    // without the move keyword it wont work
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn waiting_for_all_threads_finish() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // where join is called may affect your threads run at same time
}

// no guarantees that spawned thread will run or will run completely
fn creating_new_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}