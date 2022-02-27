fn main() {
    let mut absent_number: Option<i32> = None;
    absent_number = Some(5); // 潜在的にNoneの可能性ある

    println!("absent_number: {:#?}", absent_number);
}
