use crate::tokens::script_lang::{
    function_call_statement_token::FunctionCallStatementToken,
    variable_declartion_token::VariableDeclartionToken, CommentToken, ScriptBlock as Output,
    StatementToken,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use tree_sitter::Node;
lazy_static! {
    static ref IS_NAMED_REGEX: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UnalignedToken {
    pub raw: String,
    pub expr: String,
    pub kind: String,
    pub grammar: String,
    pub source_language: String,
    pub children: Vec<UnalignedToken>,
    pub is_root: bool,
}

pub trait UnalignedTokenTrait {
    fn new(node: Node, input_str: &str, is_bool: bool) -> Self;
    fn add_children(&mut self, node: Node, input_str: &str);
    fn is_named_kind(&self) -> bool;
    fn convert_to_unified_token(&self) -> StatementToken;
}

impl UnalignedTokenTrait for UnalignedToken {
    fn new(node: Node, input_str: &str, is_root: bool) -> Self {
        UnalignedToken {
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

    fn convert_to_unified_token(&self) -> StatementToken {
        let clone_children = <Vec<UnalignedToken> as Clone>::clone(&self.children);
        match self.kind.as_str() {
            "call_expression" => {
                let mut call_statement = FunctionCallStatementToken {
                    function_name: "".to_string(),
                    function_args: vec![],
                    raw_content: self.raw.to_owned(),
                };
                for ch in clone_children.into_iter() {
                    match ch.kind.as_str() {
                        "selector_expression" => {
                            call_statement.function_name = ch.raw.to_owned();
                        }
                        "argument_list" => {
                            call_statement.function_args = ch
                                .children
                                .iter()
                                .map(|f| VariableDeclartionToken {
                                    prefix: vec![],
                                    assignment: None,
                                    raw_content: f.raw.to_owned(),
                                    declartion_type: f.kind.to_owned(),
                                    raw_declartion_type: f.kind.to_owned(),
                                })
                                .collect::<Vec<VariableDeclartionToken>>();
                        }
                        _ => (),
                    }
                }
                return StatementToken::FunctionCallStatement(call_statement);
            },
            

            _ => StatementToken::Comment(CommentToken {
                comment: "".to_string(),
                raw_content: self.raw.to_owned(),
            }),
        }
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
