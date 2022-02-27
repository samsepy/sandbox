#[derive(PartialEq)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("あなたのIPは{}です", route(IpAddrKind::V4));
}

fn route(ip_type: IpAddrKind) -> String {
    if ip_type == IpAddrKind::V4 {
        String::from("レガシー")
    } else {
        String::from("モダン")
    }
}
