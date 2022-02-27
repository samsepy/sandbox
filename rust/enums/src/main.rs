fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("1"),
        3 => println!("3"),
        5 => println!("5"),
        7 => println!("7"),
        _ => println!("その他"),
    }
}
