struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color (
    i32,
    i32,
    i32,
);

fn build_user(username: String) -> User {
    User {
        username,
        email: String::from("no@email.com"),
        sign_in_count: 1,
        active: false,
    }
}

fn main() {
    let user1 = User {
        email: String::from("obama@me.com"),
        username: String::from("Obama"),
        active: true,
        sign_in_count: 32,
    };
    println!("{}", user1.email);

    // cant reassign
    // user1.email = String::from("me@me.com");

    let mut user2 = User {
        email: String::from("obiwan@me.com"),
        username: String::from("Obiwan"),
        active: true,
        sign_in_count: 2,
    };
    user2.username = String::from("Obiwan Knobi");
    println!("{}", user2.username);

    let user3 = build_user(String::from("Luke"));
    println!("{}", user3.username);

    let user4 = User {
        username: user3.username,
        email: user1.email,
        sign_in_count: 8,
        active: user2.active,
    };
    println!("{}", user4.username);

    let black = Color(0, 0, 0);
}
