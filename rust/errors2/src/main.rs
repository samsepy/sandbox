use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    read_username_from_file().unwrap();
    // let f = File::open("hello.txt")?; // ?演算子はResultを返す関数でしか使用が許可されない
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
