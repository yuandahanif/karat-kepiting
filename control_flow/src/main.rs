fn main() {
    let input = 7;
    let ternary_i_guess = if input == 7 { 5 } else { 7 };

    if ternary_i_guess == 5 {
        println!("input is five");
    } else if ternary_i_guess == 7 {
        println!("input is not five but seven");
    } else {
        println!("input isn't five");
    }

    // loop
    let mut itter = 1;

    let done = loop {
        itter += 1;

        if itter == 10 {
            break "done";
        }
    };

    println!("result is {done}");

    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10 - count;
        let mut remaining_space = 10 - remaining;
        let mut right_star = remaining;

        if count == 10 {
            break 'counting_up;
        }

        loop {
            remaining_space -= 1;
            print!(" ");

            if remaining_space < 0 {
                break;
            }
        }

        loop {
            remaining -= 1;
            print!("*");

            if remaining <= 0 {
                break;
            }
        }

        while right_star > 0 {
            right_star -= 1;
            print!("*");
        }
        count += 1;
        println!("");
    }

    const TAG_INDEX: [i32; 10] = [2, 3, 5, 7, 9, 11, 12, 13, 16, 20];

    for i in TAG_INDEX {
        println!("{i}");
    }
}
