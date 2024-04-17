pub mod gardening_stuff;

// using use
use crate::gardening_stuff as garden;
use crate::gardening_stuff::Plant;

fn destroy_all() {
    // relative path
    garden::funny_stuff();
}

pub mod dangerous_gardening_stuff {
    pub fn reset_all() {
        // relative using super::
        super::destroy_all();

        let mut tomato = super::Plant::plant(String::from("tomato"));
        tomato.add_pest(super::garden::PestType::Child);

        // absolute path
        crate::gardening_stuff::plant::plant(tomato);
    }
}


pub fn clone_plant(plant: &Plant) -> Plant {
    let new_plant = Plant::plant(plant.name.clone());
    new_plant
}

// re-exporting to be use by other
pub use gardening_stuff::Plant as DefaultPlant;
