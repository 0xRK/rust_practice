#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind {
    V4(IpAddr),
    V6(IpAddr),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let four = IpAddrKind::V4(home);
    let six = IpAddrKind::V6(loopback);

    println!("{:#?}", four);
    println!("{:#?}", six);
}