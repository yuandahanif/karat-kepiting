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
    let nullable_url: Option<String> = Some(String::from("localhost")); // this is also enum, with nullable value
    let v4 = IPAddrKind::V4(127, 0, 0, 1);
    let v6 = IPAddrKind::V6(String::from("::1"));

    v4.dbg();
    v6.dbg();

    // type check
    match nullable_url {
        None => {
            println!("there is no url");
        }
        Some(url) => {
            println!("the url is {}", url);
        }
    }
}
