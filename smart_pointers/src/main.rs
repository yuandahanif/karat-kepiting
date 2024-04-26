use std::rc::Rc;

use smart_pointers::cons::List::{Cons, Nil};
use smart_pointers::my_std;

fn main() {
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("Hello, world! {:?}", list);

    deref();

    let x = 5;
    let y = my_std::MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = my_std::MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // explicit; without deref coercion

    // drop trait
    let c = my_std::CustomSmartPointer {
        data: "my stuff".to_string(),
    };

    {
        let _c = my_std::CustomSmartPointer {
            data: "replacing c".to_string(),
        };
    }

    drop(c);

    let _c = my_std::CustomSmartPointer {
        data: "my new stuff".to_string(),
    };

    let _d = my_std::CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil)))));
    let b = Cons(1, Rc::clone(&a));
    println!("owner count {}", Rc::strong_count(&a));

    {
        let b = Cons(1, Rc::clone(&a));
        println!("owner count {}", Rc::strong_count(&a));
    }

    println!("owner count {}", Rc::strong_count(&a));
    let c = Cons(2, Rc::clone(&a));
    println!("owner count {}", Rc::strong_count(&a));
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
