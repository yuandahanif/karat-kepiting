use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("the largest number: {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("the largest number: {}", largest(&number_list));

    let x = String::from("let string1 = String::from(\"long string is long\");");
    let y = "kys";

    println!("longest stirng: {}", longest(x.as_str(), y));
    longest_with_an_announcement(x.as_str(), y, "daadwda");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

#[derive(PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

//  this implementation apply to all generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this implementation only apply for f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl std::cmp::PartialOrd for Point<usize> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

// function with (shortes borrowed) lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() {
        return y;
    }
    return x;
}

// mix them all
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if y.len() > x.len() {
        return y;
    }
    return x;
}
