use {
    super::fact::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TraitFact {
    AttributeAdjust(FactAttributeAdjust),
    Buff(FactBuff),
    BuffConversion(FactBuffConversion),
    ComboField(FactComboField),
    ComboFinisher(FactComboFinisher),
    Damage(FactDamage),
    Distance(FactDistance),
    NoData(FactNoData),
    Number(FactNumber),
    Percent(FactPercent),
    PrefixedBuff(FactPrefixedBuff),
    Radius(FactRadius),
    Range(FactRange),
    Recharge(FactRecharge),
    Time(FactTime),
    Unblockable(FactUnblockable),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraitedFact {
    #[serde(flatten)]
    pub fact: TraitFact,

    pub requires_trait: usize,
    pub overrides: Option<usize>,
}
