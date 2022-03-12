#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    // let integer = Point { x: 5, y: 10.0 }; NG: 型は同じでなくてはいけない
    let float = Point { x: 1.0, y: 4.0 };
    // let float = Point { x: 1.0, y: 4 }; NG: 型は同じでなくてはいけない
    println!("integer: {:?}", integer);
    println!("float: {:?}", float);
}
