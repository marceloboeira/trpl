struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("foo@bar.com"), String::from("foobar"));

    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("notthesameasbefore");

    println!("User {} logged in with {} email address", user1.username, user1.email);
}
