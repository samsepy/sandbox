fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("1"),
        Some(3) => println!("3"),
        Some(5) => println!("5"),
        Some(7) => println!("7"),
        _ => println!("その他"),
    }
}
