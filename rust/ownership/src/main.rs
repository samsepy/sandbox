fn main() {
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("hello: {}", hello); // スライスサンプル
    // println!("world: {}", world); // スライスサンプル

    let s = String::from("hello world"); // &str型
    let len = first_word(&s); // 不変借用発生

    // s.clear(); // NG: 下記不変借用で使われているのにも関わらず可変借用を利用しているためエラー

    println!("s: {}", s);
    println!("len: {}", len); // 不変借用はここで使われる
}

fn first_word(s: &str) -> &str {
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
