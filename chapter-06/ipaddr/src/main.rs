enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn without_local_use() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn with_local_use() {
    use IpAddr::*;
    let home = V4(127, 0, 0, 1);
    let loopback = V6(String::from("::1"));
}