enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Hello, world!");
}

fn route(ip_kind: IpAddr) {
    unimplemented!("???")
}