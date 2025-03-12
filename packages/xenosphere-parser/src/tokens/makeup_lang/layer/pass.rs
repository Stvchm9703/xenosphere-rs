use serde::{Deserialize, Serialize};

use crate::tokens::script_lang::ScriptBlock;


// --------------------------------
// Layer Pass
#[derive(Debug, Serialize, Deserialize)]
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

