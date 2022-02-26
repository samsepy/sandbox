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

    println!("user1 email: {}", user1.email);
    println!("user1 username: {}", user1.username);
    println!("user1 active: {}", user1.active);
    println!("user1 sign_in_count: {}", user1.sign_in_count);

    let user2 = User {
        email: String::from("hoge2@example.com"),
        username: String::from("hoge2"),
        ..user1
    };

    println!("user2 email: {}", user2.email);
    println!("user2 username: {}", user2.username);
    println!("user2 active: {}", user2.active);
    println!("user2 sign_in_count: {}", user2.sign_in_count);
}
