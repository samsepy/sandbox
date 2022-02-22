fn main() {
    let s1 = String::from("hello, world!");
    let s2 = s1;
    println!("{}", s2); // s1を呼び出そうとする場合エラーが出る
}
