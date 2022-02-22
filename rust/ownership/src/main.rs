fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // NG: ムーブが作用して変数sは利用不可

    let x = 5;
    makes_copy(x);
    println!("{}", x); // OK: ムーブが作用するがi32はCopyなので利用可

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
}
