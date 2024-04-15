#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPAddrKind {
    fn dbg(&self) {
        dbg!(&self);
    }
}

fn main() {
    let v4 = IPAddrKind::V4(127, 0, 0, 1);
    let v6 = IPAddrKind::V6(String::from("::1"));

    v4.dbg();
    v6.dbg();

    println!("Hello, world!");
}
