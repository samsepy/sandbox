use std::io;

fn main() {
    println!("数を入力してね");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");

    println!("次のように予想しました: {}", guess)
}
