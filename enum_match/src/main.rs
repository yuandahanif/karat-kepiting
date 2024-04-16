#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n + 1),
        None => None,
    }
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 6,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state: {:?}", state);
            25
        }
    }
}

fn main() {
    let five = Some(5);
    let six_or_none = plus_one(five);
    println!("{:?}", six_or_none);

    let six_or_none = plus_one(None);
    println!("{:?}", six_or_none);

    value_in_cent(Coin::Quarter(UsState::Alabama));
}
