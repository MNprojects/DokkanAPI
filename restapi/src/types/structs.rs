#![allow(non_snake_case)]
use super::enums::{ self, SortOptions };
use enums::{ Classes, Types, Rarities };
use serde::{ Deserialize, Serialize };
use ts_rs::TS;
use serde_with::skip_serializing_none;
use std::fmt;
use serde_with::{ StringWithSeparator, formats::CommaSeparator };

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/Character.ts")]
pub struct Character {
    pub name: Option<String>,
    pub title: Option<String>,
    pub maxLevel: Option<i32>,
    pub maxSALevel: Option<i32>,
    pub rarity: Option<Rarities>,
    pub class: Option<Classes>,
    #[serde(rename = "type")]
    pub characterType: Option<Types>,
    pub cost: Option<i32>,
    pub id: Option<String>,
    pub imageURL: Option<String>,
    pub leaderSkill: Option<String>,
    pub ezaLeaderSkill: Option<String>,
    pub superAttack: Option<String>,
    pub ezaSuperAttack: Option<String>,
    pub ultraSuperAttack: Option<String>,
    pub ezaUltraSuperAttack: Option<String>,
    pub passive: Option<String>,
    pub ezaPassive: Option<String>,
    pub activeSkill: Option<String>,
    pub activeSkillCondition: Option<String>,
    pub ezaActiveSkill: Option<String>,
    pub ezaActiveSkillCondition: Option<String>,
    pub transformationCondition: Option<String>,
    pub links: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub kiMeter: Option<Vec<String>>,
    pub baseHP: Option<i32>,
    pub maxLevelHP: Option<i32>,
    pub freeDupeHP: Option<i32>,
    pub rainbowHP: Option<i32>,
    pub baseAttack: Option<i32>,
    pub maxLevelAttack: Option<i32>,
    pub freeDupeAttack: Option<i32>,
    pub rainbowAttack: Option<i32>,
    pub baseDefence: Option<i32>,
    pub maxDefence: Option<i32>,
    pub freeDupeDefence: Option<i32>,
    pub rainbowDefence: Option<i32>,
    pub kiMultiplier: Option<String>,
    pub transformations: Option<Vec<Transformation>>,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/interfaces/Transformation.ts")]
pub struct Transformation {
    transformedID: String,
    transformedName: String,
    transformedClass: Classes,
    transformedType: Types,
    transformedSuperAttack: String,
    transformedEZASuperAttack: Option<String>,
    transformedUltraSuperAttack: Option<String>,
    transformedEZAUltraSuperAttack: Option<String>,
    transformedPassive: String,
    transformedEZAPassive: Option<String>,
    transformedActiveSkill: Option<String>,
    transformedActiveSkillCondition: Option<String>,
    transformedLinks: Vec<String>,
    transformedImageURL: String,
}

use std::sync::{ Arc, RwLock };

#[derive(Debug)]
pub struct AppState {
    pub characters: Arc<RwLock<Vec<Character>>>,
}

#[serde_with::serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct ApiParams {
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub name: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub fname: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub title: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub ftitle: Option<Vec<String>>,
    pub has_trasformation: Option<bool>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub links: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub categories: Option<Vec<String>>,
    pub sort_by: Option<SortOptions>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub id: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub characterType: Option<Vec<Types>>,
    pub rarity: Option<Rarities>,
    pub class: Option<Classes>,
    pub num: Option<usize>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, i32>>")]
    pub cost: Option<Vec<i32>>,
    pub reverse: Option<bool>,
}

impl std::fmt::Display for ApiParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}