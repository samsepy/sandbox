#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = 30 * 2;
        self.width * self.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1は{:#?}です", rect1);
    println!("長方形の面積は{}です", rect1.area());
}
