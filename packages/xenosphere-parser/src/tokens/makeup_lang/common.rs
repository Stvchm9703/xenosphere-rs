use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub (crate) fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttributeSet {
    /// General Attribute Set
    pub name: String,
    pub value: HashMap<String, String>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum FuncArgValue {
    Int(i32),
    Float(f32),
    String(String),
    InputReference(String),
    OutputReference(String),
    None,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FuncArgSet {
    pub name: String,
    pub value: FuncArgValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncCallSet {
    pub func_name: String,
    // None : maybe bypass the arg from last layer function call return, or by the runtime-call
    // Vec<FuncArgSet> : by by argument position,
    pub func_arg: Option<Vec<FuncArgSet>>,
}

// --------------------------------
