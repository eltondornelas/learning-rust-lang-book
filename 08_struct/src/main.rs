struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    // let user1 = User {  // without mut, the L17 will fail
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("testing");

    let user2 = build_user("test@test.com".to_string(), String::from("username_test"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("another_username"),
        ..user1  // struct update syntax. same as: active: user1.active, sign_in_count: user1.sign_in_count
    };

    tuple_structs();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // simplified from email: email
        username,  // simplified from username: username
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let int = black.2;
    println!("Black in index 2 is: {}", int);
}