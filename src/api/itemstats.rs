use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStats {
    pub id: usize,
    pub name: String,
    pub attributes: Vec<StatAttribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatAttribute {
    pub attribute: String,
    pub multiplier: f32,
    pub value: u32,
}
