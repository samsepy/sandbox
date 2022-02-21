fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("配列{}番目のの値は{}です", index, a[index]);
        index = index + 1;
    }
}
