// imports, from a crate!
// project initialization happens with `cargo new gw2buildfuzzer` for example
// a crate installed by `cargo add serde`
use serde::{Deserialize, Serialize};

/*
this tells serde (the data serialization and deserialization library i would generally recommend, alongside serde_json for any JSON data to allow deserialization and serialization of that type when found in any data input provided to serde
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStats {
    pub id: usize,
    pub name: String,
    // a Vec is a ...list?
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub attribute: String,
    pub multiplier: f32,
    pub value: u32,
}
