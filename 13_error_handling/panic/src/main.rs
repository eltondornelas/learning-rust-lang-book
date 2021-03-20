fn main() {
    // forcing panic
    // panic!("crash and burn");

    // panic coming from the library because of a bug
    let v = vec![1, 2, 3];

    v[99];
    // RUST_BACKTRACE=1 cargo run
}