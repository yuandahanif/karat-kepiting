fn main() {
    // paniccc();
    // file_read();
    // file_read_closure();
    // get_username().unwrap();
    shorter_get_username().unwrap();

    // println!("Hello, world!");
}

fn paniccc() {
    let arr = vec![1, 2, 3];

    arr[12]; // buffer overread

    panic!("woof woof!");
}

use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

fn file_read() {
    let graeting_file_result = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Err(err) => panic!("error creating file: {:?}", err),
                Ok(file) => file,
            },
            other_err => panic!("error opening file: {:?}", other_err),
        },
    };

    let graeting_file_result = File::open("hello.txt").unwrap(); // shortcut to return panic if error

    let greating_file_result =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn file_read_closure() -> File {
    File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("error creating file: {:?}", err);
            })
        } else {
            panic!("problem opening file: {:?}", err);
        }
    })
}

fn get_username() -> Result<String, Error> {
    let mut file = File::open("helo.txt");

    let mut username_file = match file {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Err(err) => Err(err),
        Ok(_) => Ok(username),
    }
}

fn shorter_get_username() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_shorter_get_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

use std::fs;

// my "honest" reaction => dawmnnnn!
fn even_more_shorter_get_username() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}
