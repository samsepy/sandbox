fn main() {
    let s = String::from("hello world");
    let len = first_word(&s);

    println!("first word length: {}", len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // println!("bytes[0]: {}", bytes[0]); // => 104: アスキーコードでhに相当
    // println!("bytes[0]: {}", b'h'); // => 104: hという文字リテラルからアスキーコードに変換

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
