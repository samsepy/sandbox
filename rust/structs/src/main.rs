fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let red = Color(255, 0, 0);

    println!("red: {}", red.0);
    println!("red: {}", red.1);
    println!("red: {}", red.2);
}
