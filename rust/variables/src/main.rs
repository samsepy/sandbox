fn main() {
    let x: u32 = "42".parse().expect("数字じゃありません");
    println!("xの値は: {}", x);
}
