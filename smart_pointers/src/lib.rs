pub mod cons {
    use std::rc::Rc;


    #[derive(Debug)]
    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
}

// my_std sound weird
pub mod my_std {
    use std::ops::Deref;

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(inp: T) -> MyBox<T> {
            MyBox(inp)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub struct CustomSmartPointer {
        pub data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
}
