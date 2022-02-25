fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let email = String::from("hoge@example.com");
    let username = String::from("hoge");

    let user1 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };


    println!("email: {}", user1.email);
    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);
}
