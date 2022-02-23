fn main() {
    let s = String::from("hello world");
    let len = first_word(&s);

    println!("first word length: {}", len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
