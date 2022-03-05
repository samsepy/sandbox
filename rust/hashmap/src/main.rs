fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        println!("word: {}", word);
        let count = map.entry(word).or_insert(0); // or_insert関数はこのキーに対する値への可変参照(&mut V)を返す
        *count += 1; // 値を代入するために参照外し
    }

    println!("{:?}", map);
}
