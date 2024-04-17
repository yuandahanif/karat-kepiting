use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let asparagus = Asparagus {};
    println!("Hello, {:?}!", asparagus);
}
