// use core::io;
use std::path::PathBuf;

use anyhow::Error;
use serde::{Deserialize, Serialize};

pub mod layer;
pub use layer::*;

pub mod common;
pub use common::*;

pub mod file_attribute;
pub use file_attribute::*;

use bincode;
use serde_json::to_string;

use crate::parsers::{makeup_lang::SyntexParser, parse_makeup_token};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct XeslFileContent {
    pub file_name: String,
    pub file_path: PathBuf,
    pub cache_path: PathBuf,
    pub content: Vec<XeslFileToken>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "_token_type")]
pub enum XeslFileToken {
    Attribute(FileAttribute),
    Layer(LayerObj),
    Unknown(String),
}

impl Default for XeslFileToken {
    fn default() -> Self {
        Self::Unknown(String::default())
    }
}

impl XeslFileContent {
    pub fn from_file(file_path: &PathBuf) -> anyhow::Result<Self, Error> {
        let content = std::fs::read_to_string(file_path)?;
        let mut result_token = parse_makeup_token(&content)?;
        result_token.file_path = file_path.to_owned();
        result_token.file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        Ok(result_token)
    }

    pub fn write_cache(&self, output_path: &PathBuf) -> anyhow::Result<(), Error> {
        let content = bincode::serialize(self)?;
        std::fs::write(output_path, content)?;
        Ok(())
    }
    pub fn write_debug_cache(&self, output_path: &PathBuf) -> anyhow::Result<(), Error> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(output_path, content)?;
        Ok(())
    }
    pub fn read_cache(input_path: &PathBuf) -> anyhow::Result<Self, Error> {
        let content = std::fs::read_to_string(input_path)?;
        let bytes = content.as_bytes();
        let token = bincode::deserialize(bytes)?;
        Ok(token)
    }
    pub fn read_debug_cache(input_path: &PathBuf) -> anyhow::Result<Self, Error> {
        let content = std::fs::read_to_string(input_path)?;
        let token = serde_json::from_str(&content)?;
        Ok(token)
    }

    pub fn print(&self) {
        println!("{:?}", to_string(self));
    }
}
