use std::collections::HashMap;

#[derive(Debug)]
struct Plant {
    name: String,
    age: u8,
}

#[derive(Debug)]
enum PlantType {
    Mushroom(Plant),
    Weed(Plant),
    Moss(Plant),
    Nuclear(Plant),
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

    // use enum
    let plants = vec![PlantType::Nuclear(Plant {
        name: String::from("rock"),
        age: 0,
    })];

    println!("all other plants {:?}", plants);

    strings();
    hashes();
}

fn strings() {
    let data = "initial content";

    let s = String::new();
    let s = String::from("initial content");
    let s = data.to_string();
    let mut s = "initial_content".to_string();

    // encode as UTF-8
    let hello = String::from("안녕하세요");
    let hello = String::from("こんにちは");

    s.push(' ');
    s.push_str(&hello.clone());

    println!("{:?}", s);

    let hello3 = format!("{hello} {hello} {hello}");
    println!("{:?}", hello3);

    for ch in hello.chars() {
        print!("{ch} - ");
    }

    for byte in hello.bytes() {
        print!("{byte} - ");
    }
}

fn hashes() {
    #[derive(Debug)]
    struct Team {
        score: u8,
    }

    let mut teams = HashMap::new();
    teams.insert("red".to_string(), Team { score: 8 });
    teams.insert("blue".to_string(), Team { score: 12 });

    teams.insert("red".to_string(), Team { score: 90 }); // replace a value

    let red = teams.get("red").clone();
    if let Some(red) = red {
        println!("{}", red.score);
    }

    // insert if not exist
    teams.entry("blue".to_string()).or_insert(Team { score: 2 });
    teams
        .entry("yellow".to_string())
        .or_insert(Team { score: 9 });

    for (k, v) in &teams {
        println!("{}: {:?}", k, v)
    }

    let mut word_count = HashMap::new();

    let text = "Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing 8-25 shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.";

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}
