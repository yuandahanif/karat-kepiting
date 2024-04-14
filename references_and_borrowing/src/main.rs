fn main() {
    let s = String::from("Hello");
    let len = calculate_len(&s);
    println!("{len}");

    let mut s = s.clone();
    add_question(&mut s);

    println!("{s}");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn add_question(s: &mut String) {
    s.push_str("?");
}
