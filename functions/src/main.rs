fn main() {
    another_fn();
    print_add(1, 5);

    // expression
    let exp_result = {
        let b = 2;
        b + 2
    };

    println!("expression vs statement: {exp_result}");

    let add_result = add(2, 3);
    println!("add result {add_result}");
}

fn another_fn() {
    println!("Hello from another function!");
}

fn print_add(x: i32, y: i32) {
    println!("{0}", x + y);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
