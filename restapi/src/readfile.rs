
use super::types::structs::Character;
use std::fs::File;
use std::io::Read;

use std::sync::{Arc, RwLock};

pub fn get_content() -> Arc<RwLock<Vec<Character>>>{
    let mut file = File::open("./test.json").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Parse the JSON contents into a vector of Character
    let characters: Vec<Character> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    print!("hello wordl");
    // Wrap the Character in an Arc<RwLock<_>> so that they can be shared between threads
    return  Arc::new(RwLock::new(characters));
   
}