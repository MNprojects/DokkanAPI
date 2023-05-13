use crate::{types::{structs::{Character, ApiParams}, enums::SortOptions}, sort_by_field};

pub fn apply_filters(params: ApiParams, characters: & Vec<Character>) -> Vec<& Character> {
    let filtered_characters = characters
        .iter()
        .filter(|&character| {
            let mut matched = true;
            if let Some(name) = &params.name {
                matched &= name.iter().any(|n| character.name.as_ref().unwrap() == n)
            }
            if let Some(fname) = &params.fname {
                matched &= fname.iter().any(|fnm| character.name.as_ref().unwrap().contains(fnm))
            }
            if let Some(class) = &params.class {
                matched &= character.class.as_ref().unwrap().as_string() == class.as_string();
            }
            if let Some(rarity) = &params.rarity {
                matched &=  character.rarity.as_ref().unwrap().as_string() ==  rarity.as_string()
            }
            if let Some(title) = &params.title {
                matched &= title.iter().any(|n| character.title.as_ref().unwrap() == n)
            }
            if let Some(ftitle) = &params.ftitle {
                matched &= ftitle.iter().any(|fnm| character.title.as_ref().unwrap().contains(fnm))
            }
            if let Some(r#type) = &params.characterType {
                matched &= r#type.iter().any(|typ| character.characterType.as_ref().unwrap().as_string() == typ.as_string())
            }
            if let Some(cost) = &params.cost {
                matched &= cost.iter().any(|c| character.cost.as_ref().unwrap() == c)
            }
            if let Some(id) = &params.id {
                matched &= id.iter().any(|c| character.id.as_ref().unwrap() == c)
            }
           
            if let Some(has_transformation) = params.has_trasformation {
                
                matched &= character.transformations.as_ref().unwrap().is_empty() != has_transformation
            }
            if let Some(categories) = &params.categories {
                matched &= categories.iter().any(|category| {
                    character.categories.as_ref().unwrap().iter().any(|c| c == category)
                })
            }
            if let Some(links) = &params.links {
                matched &= links.iter().any(|link| {
                    character.links.as_ref().unwrap().iter().any(|l| l == link)
                })
            }
            matched
        })
        .collect::<Vec<&Character>>(); // collect into a new Vec of references

    filtered_characters // return a reference to the new vector
}

pub fn apply_sort(params: ApiParams, mut characters: Vec<&Character>) -> Vec<&Character>{

    if let Some(sort_by) = &params.sort_by {
       characters.sort_by(|a, b| {
            match sort_by {
                SortOptions::Name => sort_by_field!(name, a, b),
                SortOptions::Title => sort_by_field!(title, a, b),
                SortOptions::Rarity => sort_by_field!(rarity, a, b),
                SortOptions::Class => sort_by_field!(class, a, b),
                SortOptions::Type => sort_by_field!(characterType, a, b),
                SortOptions::BaseHP => sort_by_field!(baseHP, a, b),
                SortOptions::MaxLevelHP => sort_by_field!(maxLevelHP, a, b),
                SortOptions::FreeDupeHP => sort_by_field!(freeDupeHP, a, b),
                SortOptions::RainbowHP => sort_by_field!(rainbowHP, a, b),
                SortOptions::BaseAttack => sort_by_field!(baseAttack, a, b),
                SortOptions::MaxLevelAttack => sort_by_field!(maxLevelAttack, a, b),
                SortOptions::BaseDefence => sort_by_field!(baseDefence, a, b),
                SortOptions::MaxDefence => sort_by_field!(maxDefence, a, b),
                SortOptions::FreeDupeDefence => sort_by_field!(freeDupeDefence, a, b),
                SortOptions::RainbowDefence => sort_by_field!(rainbowDefence, a, b),
                SortOptions::Cost => sort_by_field!(cost, a, b),
            }
        });
    }
    match params.reverse {
        Some(_) => {characters.reverse()}
        None => {}
    }
    characters
}