use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tokens::tensor::PseudoTensor;

use super::script_lang::ScriptBlock;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttributeSet {
    /// General Attribute Set
    pub name: String,
    pub value: HashMap<String, String>,
}

pub type LayerFileContent = Vec<LayerFileToken>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayerFileToken {
    Attribute(LayerFileAttribute),
    Layer(LayerObj),
    Unknown(String),
}

impl Default for LayerFileToken {
    fn default() -> Self {
        Self::Unknown("".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayerFileAttribute {
    // package name
    Package(LayerPackageAttr),

    // use / include / improt
    Import(LayerImportAttr),

    // export -> export function name
    Export(AttributeSet),

    Unknown(AttributeSet),
}
// -----------
// attribute list
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerPackageAttr {
    pub name: String,
    pub path: String,
}
impl LayerPackageAttr {
    pub fn from_attribute_set(attr: &AttributeSet) -> Self {
        Self {
            name: attr.name.to_owned(),
            path: attr.value.get("path").unwrap_or(&"".to_owned()).to_string(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerImportAttr {
    pub name: String,
    pub path: String,
    pub file_type: String,
}

impl LayerImportAttr {
    pub fn from_attribute_set(attr: &AttributeSet) -> Self {
        let path = if let Some(path) = attr.value.get("path") {
            path
        } else {
            ""
        };

        let file_type = if let Some(file_type) = attr.value.get("file_type") {
            file_type
        } else {
            "xesl_cpp"
        };

        Self {
            name: attr.name.to_owned(),
            path: path.to_string(),
            file_type: file_type.to_string(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerObj {
    pub name: String,
    pub property: Option<LayerProperty>,
    pub pass: Option<LayerPass>,
    pub stack: Option<LayerStacks>,
}

impl Default for LayerObj {
    fn default() -> Self {
        Self {
            name: String::default(),
            property: None,
            pass: None,
            stack: None,
        }
    }
}

// --------------------------------
// Layer Property
pub type LayerProperty = Vec<LayerPropertyElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayerPropertyElement {
    Static(LayerPropertyElementSet),
    In(LayerPropertyElementSet),
    Out(LayerPropertyElementSet),
    InDim(i32),
    OutDim(i32),
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
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
#[serde(tag = "type")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
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

// --------------------------------
// Layer Pass
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct LayerPassScript {
    // the raw content
    pub raw_content: String,

    // the transpiled token
    pub transpiled: ScriptBlock,

    // attribute `syntex`
    pub script_syntex: Option<String>,

    // attribute `filename`
    // suppose the script_filename is created by transpiler
    pub script_filename: Option<String>,

    // attribute `target`
    pub target_platform: String,

    // attribute `transpiler`
    pub transpiler: Option<String>,

    // attribute `overwrite`
    //  overwrite the previous / default language pass, but it should also not to transpile to other language
    pub is_overwrite: bool,
}

impl Default for LayerPassScript {
    fn default() -> Self {
        Self {
            raw_content: String::new(),
            transpiled: ScriptBlock::default(),
            script_syntex: None,
            script_filename: None,
            target_platform: String::new(),
            transpiler: None,
            is_overwrite: false,
        }
    }
}

pub type LayerPass = Vec<LayerPassScript>;
// --------------------------------

// --------------------------------
/// Layer Stack
pub type LayerStacks = Vec<LayerStackElm>;

/// the function block
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerStackBlock {
    pub id: usize,
    pub pos: usize,
    pub func_name: String,
    pub func_args: Vec<LayerStackFuncArgSet>,
}

pub type LayerStackFuncArgSet = FuncArgSet;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerStackBranch {
    pub id: usize,
    pub pos: usize,
    pub branch_id: usize,
    pub stack_list: Vec<LayerStackElm>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LayerStackElm {
    Block(LayerStackBlock),
    Branch(LayerStackBranch),
}

// --------------------------------
