fn main() {
    let mut s = String::from("Hello"); // string from the heap
    s.push_str(", world!");

    println!("{s}");

    // shallow and deep copy (JS flasshback)
    let s1 = String::from("new string");
    let s2 = s1; // shallow copy
    println!("{s2}");

    let s3 = s2.clone(); // deep copy
    println!("{s3}");

    // function
    let s4 = String::from("this will not valid after used as an argument");
    take_wonership(s4);

    let i1 = 32;
    copy_value(i1);

    let s5 = give_wonership();
    let s6 = takes_and_gives_back(s5);

    println!("{s6}");

    let (s7, len) = calculate_length(s6);

    println!("{s7}, {len}");
}

fn take_wonership(s: String) {
    println!("{}", s);
}

fn copy_value(i: i32) {
    println!("{i}");
}

fn give_wonership() -> String {
    String::from("this is yours now")
}

fn takes_and_gives_back(mut s: String) -> String {
    s.push_str("!");
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
