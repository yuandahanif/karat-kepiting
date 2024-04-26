pub mod cons {

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Box<List>),
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
}

