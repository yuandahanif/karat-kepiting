use smart_pointers::cons::List::{Cons, Nil};
use smart_pointers::my_std;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world! {:?}", list);

    deref();

    let x = 5;
    let y = my_std::MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = my_std::MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // explicit; without deref coercion
}

fn deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(&5, y);
    assert_eq!(5, *y);

    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(Box::new(5), y);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
