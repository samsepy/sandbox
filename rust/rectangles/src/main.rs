#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 20,
    };

    println!("rect1は{:#?}です", rect1);
    println!("長方形の面積は{}です", rect1.area());
    println!("rect2はrect1にはまりますか？: {}", rect1.can_hold(&rect2));
    println!("rect3はrect1にはまりますか？: {}", rect1.can_hold(&rect3));
}
