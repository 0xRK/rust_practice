fn main() {
    let mut user1 = build_user(String::from("someemail@example.com"), String::from("someusername123"));

    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("{}{}{}",black.0, black.1, black.2);
    println!("{}{}{}", origin.0, origin.1, origin.2);
    println!("{:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;