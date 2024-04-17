pub mod harvest;
pub mod plant;

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

pub fn funny_stuff() {}
