use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}


// cargo test --test integration_test
// cargo test --tests integration_test
// cargo test integration_test
/* all above worked... */