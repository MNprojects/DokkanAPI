use crate::{ impl_enum_as_string };
use serde::{ Deserialize, Serialize };
use ts_rs::TS;
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/interfaces/Classes.ts")]
pub enum Classes {
    Super,
    Extreme,
}
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/interfaces/Types.ts")]
pub enum Types {
    PHY,
    STR,
    AGL,
    TEQ,
    INT,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/interfaces/Rarities.ts")]
pub enum Rarities {
    UR,
    LR,
}

impl_enum_as_string!(Types { PHY, STR, AGL, TEQ, INT });
impl_enum_as_string!(Classes { Super, Extreme });
impl_enum_as_string!(Rarities { UR, LR });