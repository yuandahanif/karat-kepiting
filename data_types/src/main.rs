fn main() {
    // tupple
    let tup = (12, 13, 67);
    let (a, b, c) = tup;
    println!("deconstruct a tupple into a, b, c = {a}, {b}, {c}");
    
    let b = tup.2;
    println!("accessing directly using index {b}");

    // array
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    println!("accesing months array with index of 4 : {0}", MONTHS[4]);
}
