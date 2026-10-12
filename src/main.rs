use std::fs;
use std::io::{self, Read};

fn main() {
    let mut path = String::new();
    io::stdin().read_to_string(&mut path).unwrap();
    let path = path.trim();

    match fs::read(path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}