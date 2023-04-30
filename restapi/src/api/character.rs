use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Character {
    name: String,
    title: String,
    maxLevel: number,
    maxSALevel: number,
    rarity: Rarities,
    class: Classes,
    type: Types,
    cost: number,
    id: String,
    imageURL: String,
    leaderSkill: String,
    ezaLeaderSkill?: String,
    superAttack: String,
    ezaSuperAttack?: String,
    ultraSuperAttack?: String,
    ezaUltraSuperAttack?: String,
    passive: String,
    ezaPassive?: String,
    activeSkill?: String,
    activeSkillCondition?: String,
    ezaActiveSkill?: String,
    ezaActiveSkillCondition?: String,
    transformationCondition?: String,
    links: String[],
    categories: String[],
    kiMeter: String[],
    baseHP: number,
    maxLevelHP: number,
    freeDupeHP: number,
    rainbowHP: number,
    baseAttack: number,
    maxLevelAttack: number,
    freeDupeAttack: number,
    rainbowAttack: number,
    baseDefence: number,
    maxDefence: number,
    freeDupeDefence: number,
    rainbowDefence: number,
    kiMultiplier: String,
    transformations: Option<Array<Transformation>>
}

 enum Classes {
    Super = "Super",
    Extreme = "Extreme"
}

 enum Types {
    PHY = "PHY",
    STR = "STR",
    AGL = "AGL",
    TEQ = "TEQ",
    INT = "INT"
}
 enum Rarities {
    UR = "UR",
    LR = "LR"
}

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
    transformedLinks: Array<String>,
    transformedImageURL: String,
}