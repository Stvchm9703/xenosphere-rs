pub mod pass;
pub use pass::*;

pub mod property;
pub use property::*;

pub mod stack;
pub use stack::*;


use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LayerObj {
    pub name: String,
    pub property: Option<LayerProperty>,
    pub pass: Option<LayerPass>,
    pub stack: Option<LayerStacks>,
    pub preview: Option<String>,
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
