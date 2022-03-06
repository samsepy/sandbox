fn main() {
    println!("hello");
    panic_sample();
    println!("world"); // panic_sampleが巻き戻され実行されない
}

fn panic_sample() {
    panic!("crash and burn");
}
