use std::collections::HashMap;

use serde::{ Deserialize, Serialize};


pub(crate) fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttributeSet {
    /// General Attribute Set
    pub name: String,
    pub value: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_type", content = "value")]
pub enum FuncArgValue {
    Int(i32),
    Float(f32),
    String(String),
    InputReference(String),
    OutputReference(String),
    None,
}

// impl Serialize for FuncArgValue {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             Self::Int(i) => {
//                 let mut map = serializer.serialize_map(Some(2))?;
//                 map.serialize_entry("_type", "Int")?;
//                 map.serialize_entry("value", i)?;
//                 map.end()
//             }
//             Self::Float(f) => {
//                 let mut map = serializer.serialize_map(Some(2))?;
//                 map.serialize_entry("_type", "Float")?;
//                 map.serialize_entry("value", f)?;
//                 map.end()
//             }
//             Self::String(s) => {
//                 let mut map = serializer.serialize_map(Some(2))?;
//                 map.serialize_entry("_type", "String")?;
//                 map.serialize_entry("value", s)?;
//                 map.end()
//             }
//             Self::InputReference(s) => {
//                 let mut map = serializer.serialize_map(Some(2))?;
//                 map.serialize_entry("_type", "InputReference")?;
//                 map.serialize_entry("value", s)?;
//                 map.end()
//             }
//             Self::OutputReference(s) => {
//                 let mut map = serializer.serialize_map(Some(2))?;
//                 map.serialize_entry("_type", "OutputReference")?;
//                 map.serialize_entry("value", s)?;
//                 map.end()
//             }
//             Self::None => {
//                 let mut map = serializer.serialize_map(Some(1))?;
//                 map.serialize_entry("_type", "None")?;
//                 map.end()
//             }
//         }
//     }
// }

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
