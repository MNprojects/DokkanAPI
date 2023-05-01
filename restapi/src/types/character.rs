#![allow(non_snake_case)]
use super::enums;
use enums::{ Classes, Types, Rarities };
use serde::{ Deserialize, Serialize };
use ts_rs::TS;

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