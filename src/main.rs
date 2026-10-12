use std::fs;
use std::io::{self, Read};

fn main() {
    let mut path = String::new();
    if io::stdin().read_to_string(&mut path).is_err() {
        println!("failure");
        return;
    }

    let trimmed_path = path.trim_end_matches(['\r', '\n', '\0']).trim();

    if trimmed_path.is_empty() {
        println!("failure");
        return;
    }

    match fs::read(trimmed_path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}