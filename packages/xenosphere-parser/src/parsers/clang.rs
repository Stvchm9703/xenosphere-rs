use pest::{iterators::Pairs, Parser};

use anyhow::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parsers/clang_syn.pest"]
pub struct SyntexParser;

pub fn parse(input_str: &str) -> std::result::Result<Pairs<Rule>, Error> {
    let result_token = SyntexParser::parse(Rule::program, input_str);

    match result_token {
        Ok(pairs) => {
            return Ok(pairs);
        }
        Err(e) => Err(Error::msg(format!("Error parsing input: {:?}", e))),
    }
}
