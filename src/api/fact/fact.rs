use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactCommon {
    text: Option<String>,
    icon: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactAttributeAdjust {
    #[serde(flatten)]
    pub common: FactCommon,

    value: usize,
    target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactBuff {
    #[serde(flatten)]
    pub common: FactCommon,

    status: String,
    description: Option<String>,
    apply_count: Option<usize>,
    duration: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactBuffConversion {
    #[serde(flatten)]
    pub common: FactCommon,

    source: usize,
    percent: usize,
    target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComboField {
    Air,
    Dark,
    Fire,
    Ice,
    Light,
    Lightning,
    Poison,
    Smoke,
    Ethereal,
    Water,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactComboField {
    #[serde(flatten)]
    pub common: FactCommon,

    pub field_type: ComboField,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComboFinisher {
    Blast,
    Leap,
    Projectile,
    Whirl,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactComboFinisher {
    #[serde(flatten)]
    pub common: FactCommon,

    pub percent: usize,
    pub finisher_type: ComboFinisher,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactDamage {
    #[serde(flatten)]
    pub common: FactCommon,

    pub hit_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactDistance {
    #[serde(flatten)]
    pub common: FactCommon,

    distance: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactNoData {
    #[serde(flatten)]
    pub common: FactCommon,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactNumber {
    #[serde(flatten)]
    pub common: FactCommon,

    pub value: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactPercent {
    #[serde(flatten)]
    pub common: FactCommon,

    pub percent: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactPrefixedBuff {
    #[serde(flatten)]
    pub common: FactBuff,

    pub prefix: FactBuff,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactRadius {
    #[serde(flatten)]
    pub common: FactCommon,

    distance: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactRange {
    #[serde(flatten)]
    pub common: FactCommon,

    value: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactRecharge {
    #[serde(flatten)]
    pub common: FactCommon,

    value: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactTime {
    #[serde(flatten)]
    pub common: FactCommon,

    duration: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactUnblockable {
    #[serde(flatten)]
    pub common: FactCommon,

    value: bool,
}
