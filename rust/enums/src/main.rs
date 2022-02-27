#[derive(PartialEq)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("あなたのIPは{}、{}です", home.address, route(&home));
    println!("あなたのIPは{}、{}です", loopback.address, route(&loopback));
}

fn route(ip: &IpAddr) -> String {
    if ip.kind == IpAddrKind::V4 {
        String::from("レガシー")
    } else {
        String::from("モダン")
    }
}
