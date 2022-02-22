fn main() {
    let mut s = String::from("hello");

    let r1 = &s;        // OK
    let r2 = &s;        // OK
    // let r3 = &mut s; // NG

    println!("{}, {}", r1, r2);
}
