fn main() {
    let s1 = String::from("Здравствуйте");
    // let answer = &hello[0]; // NG: s1はマルチバイト文字列なので1バイト分のアクセスができない
    let answer = &s1[0..2]; // NG: 1バイト分のアクセスは可能
    println!("answer: {}", answer);
}
