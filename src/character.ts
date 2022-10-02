export interface Character {
    name: string,
    title: string,
    maxLevel: number,
    maxSALevel: number,
    rarity: Rarities,
    class: Classes,
    type: Types,
    cost: number,
    id: string,
    imageURL: string,
    leaderSkill: string,
    ezaLeaderSkill?: string,
    superAttack: string,
    ezaSuperAttack?: string,
    ultraSuperAttack?: string,
    ezaUltraSuperAttack?: string,
    passive: string,
    ezaPassive?: string,
    activeSkill?: string,
    activeSkillCondition?: string,
    ezaActiveSkill?: string,
    ezaActiveSkillCondition?: string,
    transformationCondition?: string,
    links: string[],
    categories: string[],
    kiMeter: string[],
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
    kiMultiplier: string,
    transformations?: Transformation[]
}

export enum Classes {
    Super = "Super",
    Extreme = "Extreme"
}

export enum Types {
    PHY = "PHY",
    STR = "STR",
    AGL = "AGL",
    TEQ = "TEQ",
    INT = "INT"
}

export enum Rarities {
    UR = "UR",
    LR = "LR"
}

export interface Transformation {
    transformedID: string,
    transformedName: string,
    transformedClass: Classes,
    transformedType: Types,
    transformedSuperAttack: string,
    transformedEZASuperAttack?: string,
    transformedUltraSuperAttack?: string,
    transformedEZAUltraSuperAttack?: string,
    transformedPassive: string,
    transformedEZAPassive?: string,
    transformedActiveSkill?: string,
    transformedActiveSkillCondition?: string,
    transformedLinks:string[],
    transformedImageURL: string,
}