enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("Hello, world!");
}

fn route(ip_kind: IpAddrKind) {
    unimplemented!("???")
}