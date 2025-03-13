// clang_base.rs
use anyhow::{Error, Result};

use regex::Regex;
use tree_sitter::Parser;
use tree_sitter_c;

use crate::tokens::script_lang::StatementToken;

use super::common::count_and_trim_start;
use super::unaligned_token::{UnalignedToken, UnalignedTokenTrait};

pub fn parse_clang_tree(input_str: &str) -> Result<UnalignedToken, Error> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_c::language()).unwrap();

    // check the raw content is wrapped with function
    // let resolved_str = check_raw_count(input_str);
    let resolved_str = input_str;

    println!("resolved_str : \n {:?}", resolved_str);
    let tree = parser.parse(&resolved_str, None).unwrap();
    let root_node = tree.root_node();

    let mut root_token: UnalignedToken = UnalignedToken::new(root_node, &resolved_str, true);
    root_token.source_language = "clang".to_string();

    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        root_token.add_children(child, &resolved_str);
    }

    Ok(root_token)
}

fn check_raw_count(input_str: &str) -> String {
    let line_list: Vec<String> = input_str
        .split('\n')
        .map(|s| s.to_string())
        .filter(|p| !p.is_empty() && !p.starts_with("\n"))
        .collect();
    let mut new_line_list: Vec<(String, usize)> = vec![];
    let mut export_line_list: Vec<String> = vec![];
    // let mut should_add_wrap_func = false;
    let check_start_pattern = Regex::new(r"^func [a-zA-Z_]+\s*\([a-zA-Z_\s,]*\)\s*()\{").unwrap();
    let check_end_pattern = Regex::new(r"return\s+.*;.*}").unwrap();
    let mut start_count: usize = 0;
    let mut end_count: usize = 0;
    let mut minimium_space = 0;

    for (idx, line) in line_list.iter().enumerate() {
        let (tmp_line, tmp_count) = count_and_trim_start(&line);
        if (idx > 0 && tmp_count < minimium_space) || idx == 0 {
            minimium_space = tmp_count;
        }
        new_line_list.push((tmp_line, tmp_count));
    }

    for (line, tmp_count) in new_line_list {
        let mut tmp_line = line.trim_start().to_string();
        if check_start_pattern.is_match(&tmp_line) && tmp_count == minimium_space {
            start_count += 1;
        } else if check_end_pattern.is_match(&tmp_line) && tmp_count == minimium_space {
            end_count += 1;
        }

        if tmp_count > minimium_space {
            for _ in 1..=(tmp_count - minimium_space) {
                tmp_line = " ".to_owned() + tmp_line.as_str();
                // tmp_line = ""
            }
        }
        export_line_list.push(tmp_line);
    }

    if start_count == 0 && start_count == end_count {
        // missing main_funtion
        export_line_list.push("return $output_set;".to_owned());
        export_line_list.push("}".to_owned());
        export_line_list.push("func InstanceWrap ($input) ($output) {".to_owned());
        export_line_list.rotate_right(1);
    }
    return export_line_list.join("\n");
}

fn convert_to_unified_token(tokens: Vec<UnalignedToken>) -> Vec<StatementToken> {
    return vec![StatementToken::Unknown];
}
