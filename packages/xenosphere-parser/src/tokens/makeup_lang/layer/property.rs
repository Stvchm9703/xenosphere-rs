use crate::tokens::{
    makeup_lang::{variant_eq, FuncCallSet},
    tensor::PseudoTensor,
};
use serde::{Deserialize, Serialize};

// use super::{variant_eq, FuncCallSet};

// --------------------------------
// Layer Property
pub type LayerProperty = Vec<LayerPropertyElementSet>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerPropertyElementSet {
    pub prefix: LayerPropertyPrefix,
    pub name: String,
    pub value: LayerPropertyElementValue,
}
impl Default for LayerPropertyElementSet {
    fn default() -> Self {
        Self {
            name: String::default(),
            value: LayerPropertyElementValue::None,
            prefix: LayerPropertyPrefix::Unknown,
        }
    }
}

impl LayerPropertyElementSet {
    pub fn new_unresolved(raw: String) -> Self {
        Self {
            value: LayerPropertyElementValue::String(raw),
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LayerPropertyPrefix {
    // static param
    Static,
    // input param
    In,
    // output params
    Out,
    // unresolved
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type", content = "value")]
pub enum LayerPropertyElementValue {
    Int(i32),                              // 0
    Float(f32),                            // 1
    String(String),                        // 2
    Array(Vec<LayerPropertyElementValue>), // 3
    Func(FuncCallSet),                     // 4
    Tensor(PseudoTensor<f32>),             // 5
    None,                                  // -1
}
impl Default for LayerPropertyElementValue {
    fn default() -> Self {
        Self::None
    }
}
impl PartialEq for LayerPropertyElementValue {
    fn eq(&self, other: &Self) -> bool {
        variant_eq(self, other)
    }
}
