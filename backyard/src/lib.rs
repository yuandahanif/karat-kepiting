pub mod gardening_stuff {
    // public enum
    pub enum PestType {
        Insect,
        Child,
        None,
    }

    // public and private struct
    pub struct Plant {
        pest: PestType,
        pub name: String,
    }

    // public struct method
    impl Plant {
        pub fn pest(&self) -> &PestType {
            &self.pest // no copy trait, IDK
        }

        pub fn add_pest(&mut self, pest: PestType) {
            self.pest = pest;
        }

        pub fn plant(name: String) -> Plant {
            Plant {
                name,
                pest: PestType::None,
            }
        }
    }

    pub mod plant {
        pub fn chose_seed() {}
        pub fn plant(p: super::Plant) {}
    }

    pub mod harverst {
        fn prepare_tools() {}
        pub fn harvest() {
            prepare_tools();
        }
        pub fn store() {}
    }

    pub fn funny_stuff() {}
}

fn destroy_all() {
    // relative path
    gardening_stuff::funny_stuff();
}

pub mod dangerous_gardening_stuff {
    pub fn reset_all() {

        // relative using super::
        super::destroy_all();
        
        let mut tomato = super::gardening_stuff::Plant::plant(String::from("tomato"));
        tomato.add_pest(super::gardening_stuff::PestType::Child);

        // absolute path
        crate::gardening_stuff::plant::plant(tomato);
    }
}
