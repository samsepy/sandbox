use std::io;
use rand::Rng;

fn main() {
    println!("こんにちは");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数字は次の通り: {}", secret_number);
    
    println!("数を入力してね");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");

    println!("次のように予想しました: {}", guess)
}
