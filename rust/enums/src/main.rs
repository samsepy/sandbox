enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // ifでは論理値を返す必要があるが、matchではどんな型でも良い
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        } // アーム
        Coin::Nickel => 5,   // アーム
        Coin::Dime => 10,    // アーム
        Coin::Quarter => 25, // アーム
    }
}

fn main() {
    let c = value_in_cents(Coin::Penny);
    println!("c: {}", c);
}
