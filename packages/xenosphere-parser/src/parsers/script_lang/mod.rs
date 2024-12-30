use anyhow::Error;

use tree_sitter::{Language, Parser};
use tree_sitter_cpp;
use tree_sitter_go;
pub mod token;
use token::{Token, TokenTrait};

// pub mod clang_syntax;

pub fn parse(input_str: &str) -> std::result::Result<(), Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let tree = parser.parse(input_str, None).unwrap();
    let root_node = tree.root_node();

    let mut token: Vec<Token> = vec![];
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        let mut tmp_token = Token::new(child, input_str);
        tmp_token.add_children(child, input_str);
        token.push(tmp_token);
    }
    
    println!("token {:#?}", token);

    Ok(())
}


pub fn parse_golang(input_str: &str) -> std::result::Result<(), Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_go::language()).unwrap();
    let tree = parser.parse(input_str, None).unwrap();
    let root_node = tree.root_node();

    let mut token: Vec<Token> = vec![];
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        let mut tmp_token = Token::new(child, input_str);
        tmp_token.add_children(child, input_str);
        token.push(tmp_token);
    }
    
    println!("token {:#?}", token);

    Ok(())
}