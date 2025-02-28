use anyhow::Error;

use tree_sitter::{Language, Parser};
use tree_sitter_cpp;
use tree_sitter_rust;
pub mod unaligned_token;
use unaligned_token::{UnalignedToken, UnalignedTokenTrait};

pub mod golang_base;
pub use golang_base::parse_golang_tree as parse_golang;

// pub mod clang_syntax;

pub fn parse(input_str: &str) -> std::result::Result<(), Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let tree = parser.parse(input_str, None).unwrap();
    let root_node = tree.root_node();

    let mut token: Vec<UnalignedToken> = vec![];
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        let mut tmp_token = UnalignedToken::new(child, input_str, true);
        tmp_token.add_children(child, input_str);
        token.push(tmp_token);
    }

    // println!("token {:#?}", token);

    Ok(())
}

pub fn parse_cpp(input_str: &str) -> std::result::Result<(), Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let tree = parser.parse(input_str, None).unwrap();

    let mut root_token = UnalignedToken::new(tree.root_node(), input_str, true);
    root_token.source_language = "cpp".to_string();
    println!("token {:#?}", root_token);

    Ok(())
}

pub fn parse_rust(input_str: &str) -> std::result::Result<(), Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::language()).unwrap();
    let tree = parser.parse(input_str, None).unwrap();

    let mut root_token = UnalignedToken::new(tree.root_node(), input_str, true);
    root_token.source_language = "cpp".to_string();
    println!("token {:#?}", root_token);

    Ok(())
}

pub fn parse_python(input_str: &str) -> std::result::Result<(), Error> {
    Ok(())
}
