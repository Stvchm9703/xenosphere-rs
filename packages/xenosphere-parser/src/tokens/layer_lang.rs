use crate::tokens::tensor::{PseudoTensor};


#[derive(Debug)]
pub enum LayerFileToken {
    Attribute(LayerFileAttribute),
    Layer(LayerObj),
    Unknown(String),
}

#[derive(Debug)]
pub enum LayerFileAttribute {
    Include(LayerIncludeAttr),
    Unknown(String),
}

#[derive(Debug)]
pub struct LayerIncludeAttr {
    pub name: String,
    pub path: String,
    pub file_type: String,
}

#[derive(Debug)]
pub struct LayerObj {
    pub name: String,
    pub property: Option<LayerProperty>,
    pub pass: Option<LayerPass>,
    pub stack: Option<LayerStacks>,
}

// --------------------------------
// Layer Property
pub type LayerProperty = Vec<LayerPropertyElement>;

#[derive(Debug)]
pub enum LayerPropertyElement {
    Static(LayerPropertyElementSet),
    In(LayerPropertyElementSet),
    Out(LayerPropertyElementSet),
    InDim(i32),
    OutDim(i32),
    Unknown(String),
}

#[derive(Debug)]
pub struct LayerPropertyElementSet {
    pub name: String,
    pub value: LayerPropertyElementValue,
}

#[derive(Debug)]
pub enum LayerPropertyElementValue {
    Int(i32),
    Float(f32),
    String(String),
    Array(Vec<LayerPropertyElementValue>),
    Func(Vec<LayerFuncArgSet>),
    Tensor(PseudoTensor<f32>),
    None,
}

// --------------------------------

// --------------------------------
// Layer Pass
#[derive(Debug)]
pub struct LayerPassScript {
    pub script: Vec<String>,
    pub script_syntex: String,
    pub script_filename: String,
    pub target_platform: String,
}

pub type LayerPass = Vec<LayerPassScript>;
// --------------------------------

// --------------------------------
/// Layer Stack
pub type LayerStacks = Vec<LayerStackElm>;

#[derive(Debug)]
pub enum LayerFuncArgValue {
    Int(i32),
    Float(f32),
    String(String),
    InputReference(String),
    OutputReference(String),
}
#[derive(Debug)]
pub struct LayerFuncArgSet {
    pub name: String,
    pub value: LayerFuncArgValue,
}

/// the function block
///
#[derive(Debug)]
pub struct LayerStackBlock {
    pub func_name: String,
    pub func_args: Vec<LayerFuncArgSet>,
}

#[derive(Debug)]
pub enum LayerStackElm {
    Block(LayerStackBlock),
    Branch(Vec<LayerStackElm>),
}
// --------------------------------
