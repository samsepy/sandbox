fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:#?}", map);
    // println!("field_name: {}", field_name); // NG: hashmapにムーブされてるため参照できない
    // println!("field_value: {}", field_value); // NG: hashmapにムーブされてるため参照できない
}
