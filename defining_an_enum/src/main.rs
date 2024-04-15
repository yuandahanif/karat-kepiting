enum IPAddrKind {
    V4,
    V6,
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
