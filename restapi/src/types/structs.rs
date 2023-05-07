#![allow(non_snake_case)]
use super::enums::{self};
use enums::{ Classes, Types, Rarities };
use serde::{ Deserialize, Serialize };
use ts_rs::TS;
use serde_with::skip_serializing_none;
use std::fmt;
use serde_with::{StringWithSeparator, formats::CommaSeparator};


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/Character.ts")]
pub struct Character {
    name: String,
    title: String,
    maxLevel: i32,
    maxSALevel: i32,
    rarity: Rarities,
    class: Classes,
    #[serde(rename = "type")]
    characterType: Types,
    cost: i32,
    id: String,
    imageURL: String,
    leaderSkill: String,
    ezaLeaderSkill: Option<String>,
    superAttack: String,
    ezaSuperAttack: Option<String>,
    ultraSuperAttack: Option<String>,
    ezaUltraSuperAttack: Option<String>,
    passive: String,
    ezaPassive: Option<String>,
    activeSkill: Option<String>,
    activeSkillCondition: Option<String>,
    ezaActiveSkill: Option<String>,
    ezaActiveSkillCondition: Option<String>,
    transformationCondition: Option<String>,
    links: Vec<String>,
    categories: Vec<String>,
    kiMeter: Vec<String>,
    baseHP: i32,
    maxLevelHP: i32,
    freeDupeHP: i32,
    rainbowHP: i32,
    baseAttack: i32,
    maxLevelAttack: i32,
    freeDupeAttack: i32,
    rainbowAttack: i32,
    baseDefence: i32,
    maxDefence: i32,
    freeDupeDefence: i32,
    rainbowDefence: i32,
    kiMultiplier: String,
    transformations: Option<Vec<Transformation>>,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/interfaces/Transformation.ts")]
struct Transformation {
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
#[derive(Debug,Serialize,Deserialize)]
#[non_exhaustive]
pub struct ApiParams {
    pub name: Option<String>,
    pub fname: Option<String>,
    pub title: Option<String>,
    pub ftitle: Option<String>,
    pub has_trasformation: Option<bool>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub links: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub categories: Option<Vec<String>>,
    pub id: Option<i32>,
    #[serde(rename = "type")]
    pub characterType: Option<Types>,
    pub rarity: Option<Rarities>,
    pub class: Option<Classes>,
}
impl std::fmt::Display for ApiParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}