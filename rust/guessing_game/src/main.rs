use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("こんにちは");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数字は次の通り: {}", secret_number);

    loop {
        println!("数を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        let guess: u32 = guess.trim().parse().expect("数値を入力してください");

        println!("次のように予想しました: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい！"),
            Ordering::Greater => println!("大きい！"),
            Ordering::Equal => println!("当たり！"),
        }
    }
}
