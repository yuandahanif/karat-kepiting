pub mod gui {
    pub trait Draw {
        fn draw(&self) {
            todo!();
        }
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for comp in self.components.iter() {
                comp.draw();
            }
        }
    }
}

pub mod components {
    use crate::gui;

    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl gui::Draw for Button {
        fn draw(&self) {
            println!("{:?}", self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}
