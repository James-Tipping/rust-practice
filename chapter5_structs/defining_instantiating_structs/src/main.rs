fn main() {
    println!("Hello, world!");

    let user1 = build_user(
        String::from("John Smith"),
        String::from("john.smith@email.com"),
    );

    let user2 = User {
        active: true,
        ..user1
    };

    println!("user1 fields after spread operator to create user2");

    // let user3 = User {
    //     active: true,
    //     ..user1
    // };

    println!("active: {}", user1.active);
    // these two lines fail as String does not implement the copy trait
    // println!("username: {}", user1.username);
    // println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
