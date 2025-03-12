use serde::{Deserialize, Serialize};

// --------------------------------
// Layer Pass
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerPreview {
    // the raw content
    pub raw_content: String,

    pub dim: Vec<u8>,
}

impl Default for LayerPreview {
    fn default() -> Self {
        Self {
            raw_content: String::new(),
            dim: vec![],
        }
    }
}
