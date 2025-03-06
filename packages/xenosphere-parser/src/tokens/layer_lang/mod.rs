use crate::tokens::tensor::PseudoTensor;

use super::script_lang::ScriptBlock;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[derive(Debug, Default)]
pub struct AttributeSet {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub enum LayerFileToken {
    Attribute(LayerFileAttribute),
    Layer(LayerObj),
    Unknown(String),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct LayerPackageAttr {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct LayerImportAttr {
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
    Func(FuncCallSet),
    Tensor(PseudoTensor<f32>),
    None,
}
impl PartialEq for LayerPropertyElementValue {
    fn eq(&self, other: &Self) -> bool {
        variant_eq(self, other)
    }
}

#[derive(Debug)]
pub enum FuncArgValue {
    Int(i32),
    Float(f32),
    String(String),
    InputReference(String),
    OutputReference(String),
    None,
}
#[derive(Debug)]
pub struct FuncArgSet {
    pub name: String,
    pub value: FuncArgValue,
}

#[derive(Debug)]
pub struct FuncCallSet {
    pub func_name: String,
    // None : maybe bypass the arg from last layer function call return, or by the runtime-call 
    // Vec<FuncArgSet> : by by argument position,
    pub func_arg: Option<Vec<FuncArgSet>>,
}

// --------------------------------

// --------------------------------
// Layer Pass
#[derive(Debug)]
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
///
#[derive(Debug, Default)]
pub struct LayerStackBlock {
    pub id: usize,
    pub pos: usize,
    pub func_name: String,
    pub func_args: Vec<LayerStackFuncArgSet>,
}

pub type LayerStackFuncArgSet = FuncArgSet;

#[derive(Debug, Default)]
pub struct LayerStackBranch {
    pub id: usize,
    pub pos: usize,
    pub branch_id: usize,
    pub stack_list: Vec<LayerStackElm>,
}

#[derive(Debug)]
pub enum LayerStackElm {
    Block(LayerStackBlock),
    Branch(LayerStackBranch),
}

// --------------------------------
