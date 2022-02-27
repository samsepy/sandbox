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

    println!("あなたのIPは{}です", route(&home));
    println!("あなたのIPは{}です", route(&loopback));
}

fn route(ip: &IpAddr) -> String {
    if ip.kind == IpAddrKind::V4 {
        let s = ip.address.to_owned() + "、" + "レガシー";
        String::from(s)
    } else {
        let s = ip.address.to_owned() + "、" + "モダン";
        String::from(s)
    }
}
