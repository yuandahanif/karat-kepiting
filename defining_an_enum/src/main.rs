enum IPAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let v4 = IPAddrKind::V4;
    let v6 = IPAddrKind::V6;

    println!("Hello, world!");
}
