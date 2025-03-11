use serde::{Deserialize, Serialize};

use crate::tokens::makeup_lang::FuncArgSet;



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
#[serde(tag = "_type")]
pub enum LayerStackElm {
    Block(LayerStackBlock),
    Branch(LayerStackBranch),
}

// --------------------------------
