enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // ifでは論理値を返す必要があるが、matchではどんな型でも良い
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let c = value_in_cents(Coin::Penny);
    println!("c: {}", c);
}
