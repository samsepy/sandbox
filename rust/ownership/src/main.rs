fn main() {
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("hello: {}", hello); // スライスサンプル
    // println!("world: {}", world); // スライスサンプル

    let mut s = String::from("hello world");
    let len = first_word(&s); // 不変借用発生

    // s.clear(); // NG: 不変借用の後可変借用が発生しているためエラー

    println!("s: {}", s);
    println!("len: {}", len);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // println!("bytes[0]: {}", bytes[0]); // => 104: アスキーコードでhに相当
    // println!("bytes[0]: {}", b'h'); // => 104: hという文字リテラルからアスキーコードに変換

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
