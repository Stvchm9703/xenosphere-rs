use serde::{Deserialize, Serialize};

use crate::tokens::{makeup_lang::{variant_eq, FuncCallSet}, tensor::PseudoTensor};

// use super::{variant_eq, FuncCallSet};


// --------------------------------
// Layer Property
pub type LayerProperty = Vec<LayerPropertyElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum LayerPropertyElement {
    Static(LayerPropertyElementSet),
    In(LayerPropertyElementSet),
    Out(LayerPropertyElementSet),
    InDim(i32),
    OutDim(i32),
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub struct LayerPropertyElementSet {
    pub name: String,
    pub value: LayerPropertyElementValue,
}
impl Default for LayerPropertyElementSet {
    fn default() -> Self {
        Self {
            name: String::default(),
            value: LayerPropertyElementValue::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum LayerPropertyElementValue {
    Int(i32),
    Float(f32),
    String(String),
    Array(Vec<LayerPropertyElementValue>),
    Func(FuncCallSet),
    Tensor(PseudoTensor<f32>),
    None,
}
impl PartialEq for LayerPropertyElementValue {
    fn eq(&self, other: &Self) -> bool {
        variant_eq(self, other)
    }
}