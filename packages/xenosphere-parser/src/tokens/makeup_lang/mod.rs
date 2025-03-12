use serde::{Deserialize, Serialize};

pub mod layer;
pub use layer::*;

pub mod common;
pub use common::*;

pub mod file_attribute;
pub use file_attribute::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XeslFileContent {
    pub content: Vec<XeslFileToken>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_token_type")]
pub enum XeslFileToken {
    Attribute(FileAttribute),
    Layer(LayerObj),
    Unknown(String),
}

impl Default for XeslFileToken {
    fn default() -> Self {
        Self::Unknown(String::default())
    }
}
