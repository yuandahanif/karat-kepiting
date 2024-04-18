#[derive(Debug)]
struct Plant {
    name: String,
    age: u8,
}

enum Type {
    Mushroom,
    Weed,
    Moss,
    Nuclear,
}

fn main() {
    let plants: Vec<Plant> = Vec::new();
    let mut plants = vec![
        Plant {
            name: String::from("tomato"),
            age: 0,
        },
        Plant {
            name: String::from("firemelon"),
            age: 0,
        },
    ];

    plants.push(Plant {
        name: String::from("eggplant"),
        age: 0,
    });

    let tomato = plants.get(0);
    match tomato {
        Some(t) => println!("{}", t.name),
        _ => (),
    }

    println!("first plant {:?}", plants[0]);

    for plant in &mut plants {
        plant.age += 1;
        println!("name: {} age: {}", plant.name, plant.age);
    }

    println!("all plants {:?}", plants);
}