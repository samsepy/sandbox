#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let integer = Point { x: 5, y: 10.0 };
    let float = Point { x: 1.0, y: 4 };
    println!("integer: {:?}", integer);
    println!("float: {:?}", float);
    println!("p.x = {}", integer.x());
    println!("p.y = {}", integer.y());
    println!("p.x = {}", float.x());
    println!("p.y = {}", float.y());
}
