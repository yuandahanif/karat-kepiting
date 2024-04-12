fn main() {
    let mut x = 6;
    println!("the value of x is {x}");

    x = 5;
    println!("the value of x is now {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("this cant be change {THREE_HOURS_IN_SECONDS}");

    let y = 12;
    let y = y + 1;
    {
        let y = 12 + 12;
        println!("The value of y on this shadow realm is {y}");
    }
    println!("The value of y is {y}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces length is {spaces}");
}
