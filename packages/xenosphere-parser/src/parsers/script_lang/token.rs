use crate::tokens::script_lang::ScriptBlock as Output;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use tree_sitter::{Node, TreeCursor};
lazy_static! {
    static ref IS_NAMED_REGEX: Regex = Regex::new(r"^[\w]+$").unwrap();
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub raw: String,
    pub expr: String,
    pub kind: String,
    pub grammar: String,
    pub source_language: String,
    pub children: Vec<Token>,
    pub is_root: bool,
}

pub trait TokenTrait {
    fn new(node: Node, input_str: &str, is_bool: bool) -> Self;
    fn add_children(&mut self, node: Node, input_str: &str);
    fn is_named_kind(&self) -> bool;
}

impl TokenTrait for Token {
    fn new(node: Node, input_str: &str, is_root: bool) -> Self {
        Token {
            raw: node.utf8_text(input_str.as_bytes()).unwrap().to_owned(),
            expr: node.to_sexp(),
            kind: node.kind().to_string(),
            grammar: node.grammar_name().to_string(),
            source_language: String::from(""),
            children: vec![],
            is_root: is_root,
        }
    }
    fn add_children(&mut self, node: Node, input_str: &str) {
        let mut node_cursor = node.walk();
        if node.child_count() == 0 {
            return;
        }
        for child in node.children(&mut node_cursor) {
            let mut child_token = Token::new(child, input_str, false);
            child_token.add_children(child, input_str);
            self.children.push(child_token);
        }
    }
    fn is_named_kind(&self) -> bool {
        // let is_named_regex: Regex = Regex::new(r"^[\w]+$").unwrap();
        IS_NAMED_REGEX.is_match(&self.kind)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_named_kind() {
            if self.children.len() > 0 {
                return f
                    .debug_tuple(&self.kind)
                    .field(&self.raw.replace("\n", " "))
                    .field(&self.children)
                    .finish();
            } else {
                return f
                    .debug_tuple(&self.kind)
                    .field(&self.raw.replace("\n", " "))
                    .finish();
            }
        } else {
            write!(f, "")
        }
    }
}

pub trait UnitedTokenConverter {
    fn new(script_syntax: &str, target_platform: &str, source_file_name: &str) -> Self;
    fn parse_content(&mut self, raw_content: &str) -> Result<&Output, anyhow::Error>;
    fn convert(&mut self, token: Vec<Token>) -> Result<&Output, anyhow::Error>;
}
