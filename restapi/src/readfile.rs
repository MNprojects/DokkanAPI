use super::types::structs::Character;
use std::fs::File;
use std::io::Read;

use std::sync::{ Arc, RwLock };

pub fn get_content(file_name: String) -> Arc<RwLock<Vec<Character>>> {

    let mut file: File = File::open(format!("../data/{}", file_name.as_str())).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse the JSON contents into a vector of Character
    let characters: Vec<Character> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    // Wrap the Character in an Arc<RwLock<_>> so that they can be shared between threads
    return Arc::new(RwLock::new(characters));
}