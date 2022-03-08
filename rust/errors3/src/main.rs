use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("こんにちは");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("数を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("1から100の間の数を入力してください");
            continue;
        }

        println!("次のように予想しました: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい！"),
            Ordering::Greater => println!("大きい！"),
            Ordering::Equal => {
                println!("当たり！");
                break;
            }
        }
    }
}
