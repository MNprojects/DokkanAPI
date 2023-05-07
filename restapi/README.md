# API Documentation

#### Current Entities

`Character`
```rs
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
```
You can see all the enums is structs used in [struct.rs](https://github.com/feijoes/DokkanAPI/blob/main/restapi/src/types/structs.rs) and in [enums.rs](https://github.com/feijoes/DokkanAPI/blob/main/restapi/src/types/enums.rs)

### URLs
The current URL for the api is `http://localhost:1000/api/v1/`

At methor GET should return a list of all characters 
> Example JSON returned
```json
[
  {
    "name": "Broly & Cheelai & Lemo", 
    "title": "A New Life on Vampa",
    "maxLevel": 150,
    "maxSALevel": 20,
    "rarity": "LR",
    "class": "Extreme",
    "type": "PHY",
    "cost": 77,
    "id": "11900",
    "imageURL": "https://vignette.wikia.nocookie.net/dbz-dokkanbattle/images/9/9e/Card_1019000_thumb_apng.png/revision/latest?cb=20200217115345&format=original",
    "leaderSkill": "\"Movie Bosses\" Category Ki +4 and HP, ATK & DEF +130%;or Type Ki +4 and HP, ATK & DEF +100%",
    "superAttack": "Greatly raises DEF for 1 turn[1], causes colossal damage to enemy and lowers ATK[2]",
    "ultraSuperAttack": "Greatly raises ATK & DEF for 1 turn[3] and causes mega-colossal damage to enemy",
    "passive": "ATK & DEF +15% per Ki Sphere obtained; plus an additional ATK & DEF +5% and Ki +2 per Ki Sphere with 2 or more  Ki Spheres obtained; all allies' ATK +39% with 2 or more  or  Ki Spheres obtained; all allies' DEF +39% with 2 or more  or  Ki Spheres obtained; evades enemy's attack (including Super Attack) with 7 or more Ki Spheres obtained",
    "links": [
      "Brainiacs",
      "Solid Support",
      "Cold Judgment",
      "Big Bad Bosses",
      "Shocking Speed",
      "Fierce Battle",
      "Legendary Power"
    ],
    "categories": ["Movie Bosses", "Joined Forces", "Bond of Friendship"],
    "kiMeter": ["KiMeter 2 Green LR"],
    "baseHP": 6537,
    "maxLevelHP": 21575,
    "freeDupeHP": 23575,
    "rainbowHP": 26975,
    "baseAttack": 4628,
    "maxLevelAttack": 15275,
    "freeDupeAttack": 17275,
    "rainbowAttack": 20275,
    "baseDefence": 2346,
    "maxDefence": 7744,
    "freeDupeDefence": 9744,
    "rainbowDefence": 12344,
    "kiMultiplier": "12 Ki Multiplier is 150%; 24 Ki Multiplier is 200%; SA Lv.20 raises SA Multiplier by an additional 30%",
    "transformations": []
  }
]
```
### API params
You can filter or sort the character for this categories at `http://localhost:1000/api/v1/?Param=value`

Field | Description | example
------|------------ | ------
name | The exact name of the character. You can pass multiple names separated with a "," | `/api/v1/?name=Super%20Saiyan%202%20Vegeta%20(Angel),Broly&20%26&20Cheelai%20%26&20Lemo`

