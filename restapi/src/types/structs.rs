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
    name: Option<String>,
    title: Option<String>,
    maxLevel: Option<i32>,
    maxSALevel: Option<i32>,
    rarity: Option<Rarities>,
    class: Option<Classes>,
    #[serde(rename = "type")]
    characterType: Option<Types>,
    cost: Option<i32>,
    id: Option<String>,
    imageURL: Option<String>,
    leaderSkill: Option<String>,
    ezaLeaderSkill: Option<String>,
    superAttack: Option<String>,
    ezaSuperAttack:Option<String>,
    ultraSuperAttack:Option<String>,
    ezaUltraSuperAttack:Option<String>,
    passive: Option<String>,
    ezaPassive: Option<String>,
    activeSkill: Option<String>,
    activeSkillCondition: Option<String>,
    ezaActiveSkill: Option<String>,
    ezaActiveSkillCondition: Option<String>,
    transformationCondition: Option<String>,
    links: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    kiMeter: Option<Vec<String>>,
    baseHP: Option<i32>,
    maxLevelHP: Option<i32>,
    freeDupeHP: Option<i32>,
    rainbowHP: Option<i32>,
    baseAttack: Option<i32>,
    maxLevelAttack: Option<i32>,
    freeDupeAttack: Option<i32>,
    rainbowAttack: Option<i32>,
    baseDefence: Option<i32>,
    maxDefence: Option<i32>,
    freeDupeDefence: Option<i32>,
    rainbowDefence: Option<i32>,
    kiMultiplier: Option<String>,
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
#[derive(Debug, Serialize, Deserialize)]
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
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, i32>>")]
    pub num: Option<Vec<i32>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, i32>>")]
    pub cost: Option<Vec<i32>>,
    pub reverse: Option<bool>,
}
// TODO
/*
#[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")] in type
 */
impl std::fmt::Display for ApiParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}