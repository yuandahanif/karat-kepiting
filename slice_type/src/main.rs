fn main() {
    let s = String::from("Hello, world!");
    let first_word_1 = first_word(&s);
    let first_word_2 = first_word_str(&s);
    println!("{first_word_1} {first_word_2}");
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> &str {
    first_word_str(&s[..])
}
