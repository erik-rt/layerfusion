use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<BTreeMap<String, String>>,
}

impl Metadata {
    pub fn new(
        name: String,
        description: String,
        image: String,
        attributes: Vec<BTreeMap<String, String>>,
    ) -> Self {
        Metadata {
            name,
            description,
            image,
            attributes,
        }
    }
}
