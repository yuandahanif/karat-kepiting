enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let v4 = IPAddrKind::V4(127, 0, 0, 1);
    let v6 = IPAddrKind::V6(String::from("::1"));

    println!("Hello, world!");
}
