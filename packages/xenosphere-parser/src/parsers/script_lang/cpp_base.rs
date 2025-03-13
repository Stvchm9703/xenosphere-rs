// cpp_base.rs
use anyhow::{Error, Result};

// use regex::Regex;
use tree_sitter::Parser;
use tree_sitter_cpp;

use crate::tokens::script_lang::StatementToken;

// use super::common::count_and_trim_start;
use super::unaligned_token::{UnalignedToken, UnalignedTokenTrait};

pub fn parse_cpp_tree(input_str: &str) -> Result<UnalignedToken, Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();

    // check the raw content is wrapped with function
    // let resolved_str = check_raw_count(input_str);
    let resolved_str = input_str;

    // println!("resolved_str : \n {:?}", resolved_str);
    let tree = parser.parse(&resolved_str, None).unwrap();
    let root_node = tree.root_node();

    let mut root_token: UnalignedToken = UnalignedToken::new(root_node, &resolved_str, true);
    root_token.source_language = "cpp".to_string();

    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        root_token.add_children(child, &resolved_str);
    }

    // #todo: implement the unaligned-token to StatementToken
    Ok(root_token)
}

fn convert_to_unified_token(tokens: Vec<UnalignedToken>) -> Vec<StatementToken> {
    return vec![StatementToken::Unknown];
}
