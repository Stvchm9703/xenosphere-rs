use crate::tokens::script_lang::ScriptBlock as Output;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
pub extern crate tree_sitter;
use tree_sitter::Node as TNode;
lazy_static! {
    static ref IS_NAMED_REGEX: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
    static ref IS_SYMBOL_REGEX: Regex = Regex::new(r"^[\{\}\.\,\[\]\(\)\<\>]$").unwrap();
}

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnalignedToken {
    pub raw: String,
    pub expr: String,
    pub kind: String,
    pub grammar: String,
    pub source_language: String,
    pub children: Vec<UnalignedToken>,
    pub is_root: bool,
    pub start_pos: (usize, usize),
    pub end_pos: (usize, usize),
}

pub trait UnalignedTokenTrait {
    fn new(node: TNode, input_str: &str, is_bool: bool) -> Self;
    fn add_children(&mut self, node: TNode, input_str: &str);
    fn is_named_kind(&self) -> bool;
    fn is_symbol(&self) -> bool;
}

impl UnalignedTokenTrait for UnalignedToken {
    fn new(tnode: TNode, input_str: &str, is_root: bool) -> Self {
        let start_pos = tnode.start_position();
        let end_pos = tnode.end_position();
        UnalignedToken {
            raw: tnode.utf8_text(input_str.as_bytes()).unwrap().to_owned(),
            expr: tnode.to_sexp(),
            kind: tnode.kind().to_string(),
            grammar: tnode.grammar_name().to_string(),
            source_language: String::from(""),
            children: vec![],
            start_pos: (start_pos.row, start_pos.column),
            end_pos: (end_pos.row, end_pos.column),
            is_root,
        }
    }
    fn add_children(&mut self, tnode: TNode, input_str: &str) {
        let mut node_cursor = tnode.walk();
        if tnode.child_count() == 0 {
            return;
        }
        for child in tnode.children(&mut node_cursor) {
            let mut child_token = UnalignedToken::new(child, input_str, false);
            child_token.source_language = self.source_language.to_owned();
            child_token.add_children(child, input_str);
            self.children.push(child_token);
        }
    }
    fn is_named_kind(&self) -> bool {
        // let is_named_regex: Regex = Regex::new(r"^[\w]+$").unwrap();
        IS_NAMED_REGEX.is_match(&self.kind)
    }

    fn is_symbol(&self) -> bool {
        IS_SYMBOL_REGEX.is_match(&self.kind)
    }
}

impl Debug for UnalignedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // if self.is_root {
        //     return write!(f, "");
        // }
        if self.is_named_kind() == false {
            return write!(f, "");
        }

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
    }
}

pub trait UnitedTokenConverter {
    fn new(script_syntax: &str, target_platform: &str, source_file_name: &str) -> Self;
    fn parse_content(&mut self, raw_content: &str) -> Result<&Output, anyhow::Error>;
    fn convert(&mut self, token: Vec<UnalignedToken>) -> Result<&Output, anyhow::Error>;
}
