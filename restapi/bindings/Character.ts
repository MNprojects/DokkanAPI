// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Classes } from "./Classes";
import type { Rarities } from "./Rarities";
import type { Transformation } from "./Transformation";
import type { Types } from "./Types";

export interface Character { name: string, title: string, maxLevel: number, maxSALevel: number, rarity: Rarities, class: Classes, type: Types, cost: number, id: string, imageURL: string, leaderSkill: string, ezaLeaderSkill: string | null, superAttack: string, ezaSuperAttack: string | null, ultraSuperAttack: string | null, ezaUltraSuperAttack: string | null, passive: string, ezaPassive: string | null, activeSkill: string | null, activeSkillCondition: string | null, ezaActiveSkill: string | null, ezaActiveSkillCondition: string | null, transformationCondition: string | null, links: Array<string>, categories: Array<string>, kiMeter: Array<string>, baseHP: number, maxLevelHP: number, freeDupeHP: number, rainbowHP: number, baseAttack: number, maxLevelAttack: number, freeDupeAttack: number, rainbowAttack: number, baseDefence: number, maxDefence: number, freeDupeDefence: number, rainbowDefence: number, kiMultiplier: string, transformations: Array<Transformation> | null, }