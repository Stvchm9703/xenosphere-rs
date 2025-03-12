pub mod pass;
pub use pass::*;

pub mod property;
pub use property::*;

pub mod stack;
pub use stack::*;

pub mod preview;
pub use preview::*;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LayerObj {
    pub name: String,
    pub property: Option<LayerProperty>,
    pub pass: Option<LayerPass>,
    pub stack: Option<LayerStacks>,
    pub preview: Option<LayerPreview>,
}

impl Default for LayerObj {
    fn default() -> Self {
        Self {
            name: String::default(),
            property: None,
            pass: None,
            stack: None,
            preview: None,
        }
    }
}
