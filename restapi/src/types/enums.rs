use crate::impl_enum_as_string;
use serde::{ Deserialize, Serialize };
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Classes {
    Super,
    Extreme
}
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Types {
    PHY,
    STR,
    AGL,
    TEQ,
    INT
}
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Rarities {
    UR,
    LR
}
impl_enum_as_string!(Types { PHY, STR, AGL, TEQ, INT });
impl_enum_as_string!(Classes { Super, Extreme });
impl_enum_as_string!(Rarities { UR, LR });
