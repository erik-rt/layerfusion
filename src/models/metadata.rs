use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

impl Metadata {
    pub fn new(
        name: String,
        description: String,
        image: String,
        attributes: Vec<Attribute>,
    ) -> Self {
        Metadata {
            name,
            description,
            image,
            attributes,
        }
    }
}
