static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

pub fn unsafe_rust_main() {
    // dereference_raw_pointer();
    // unsafe_function_method();
    // safe_abstraction_over_unsafe_code();
    // call_external_code();
    // static_variable();
    unsafe_trait();
}

fn unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

fn static_variable() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn call_external_code() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
}

fn safe_abstraction_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);  // standard library method
    let (a, b) = split_at_mut(r, 3);

    println!("{:?}", a);
    println!("{:?}", b);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    use std::slice;

    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_function_method() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn dereference_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;  // immutable raw pointer
    let r2 = &mut num as *mut i32;  // mutable raw pointer

    unsafe {
        // to dereference de raw pointer it must be inside a unsafe block
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe {
    //     println!("{}", *r)
    // }
}