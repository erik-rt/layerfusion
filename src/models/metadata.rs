use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<BTreeMap<String, String>>,
}

impl Metadata {
    fn new(
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
