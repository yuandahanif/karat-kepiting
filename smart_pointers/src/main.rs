use std::cell::RefCell;
use std::rc::{Rc, Weak};

use smart_pointers::cons::List::{Cons, Nil};
use smart_pointers::my_std;

fn main() {
    let list = Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    );
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

    let a = Rc::new(RefCell::new(1));

    let b = Rc::new(Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(Rc::clone(&a), Rc::new(Nil))),
    ));
    println!("owner count {}", Rc::strong_count(&a));

    {
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&b));
        println!("owner count {}", Rc::strong_count(&a));
    }

    println!("owner count {}", Rc::strong_count(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&b));
    println!("owner count {}", Rc::strong_count(&a));

    dbg!(c);
    *a.borrow_mut() += 10;

    dbg!(b);
    dbg!(a);

    node_cycle();
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

fn node_cycle() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        childern: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        childern: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            childern: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
