use {
    super::fact::{TraitFact, TraitedFact},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TraitSlot {
    Major,
    Minor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GW2Trait {
    pub id: usize,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub tier: usize,
    pub order: usize,
    pub slot: TraitSlot,
    pub facts: Vec<TraitFact>,
    pub traited_facts: Vec<TraitedFact>,
    pub skills: Vec<TraitSkill>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraitSkill {
    pub id: usize,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub facts: Vec<TraitFact>,
    pub traited_facts: Vec<TraitedFact>,
}
