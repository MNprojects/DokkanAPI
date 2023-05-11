use crate::types::structs::{Character, ApiParams};

pub fn apply_filters(params: ApiParams, characters: & Vec<Character>) -> Vec<& Character> {
    let filtered_characters = characters
        .iter()
        .filter(|&character| {
            let mut matched = true;
            if let Some(name) = &params.name {
                matched &= name.contains(&character.name.as_ref().unwrap())
            }
            if let Some(class) = &params.class {
                matched &= character.class.as_ref().unwrap().as_string() == class.as_string();
            }
            
            matched
        })
        .collect::<Vec<&Character>>(); // collect into a new Vec of references

    filtered_characters // return a reference to the new vector
}