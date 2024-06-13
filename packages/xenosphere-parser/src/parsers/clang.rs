
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "parsers/clang_syn.pest"]
pub struct SyntexParser;
