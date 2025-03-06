use anyhow::Error;
use pest::iterators::Pairs;
use regex::Regex;
// use pest::Parser;
// use pest_derive::Parser;
// use syn::token::Type;
// use polars::prelude::*;

use crate::parsers::makeup_lang::Rule;
use crate::tokens::layer_lang::AttributeSet;

pub fn parse_attribute_set(pairs: Pairs<Rule>) -> Result<Vec<AttributeSet>, Error> {
    let mut attrs: Vec<AttributeSet> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::attr_set => {
                attrs.push(parse_attribute_set_content(pair.as_span().as_str())?);
            }
            _ => {}
        }
    }
    return Ok(attrs);
}

fn parse_attribute_set_content(content: &str) -> Result<AttributeSet, Error> {
    let mut attr_set = AttributeSet::default();

    let kv_patt = Regex::new(r##"(?<name>\w+)="?(?<value>[\w\(\)]+)"?"##).unwrap();
    let fn_patt = Regex::new(r##"(?<name>\w+)\((?<value>.*)\)"##).unwrap();
    let attr_patt = Regex::new(r##"^(?<name>\w+)(?<value>=true|=false)?$"##).unwrap();

    if kv_patt.is_match(content) {
        let cap = kv_patt.captures(content).unwrap();
        attr_set.name = cap["name"].to_string();
        attr_set.value = cap["value"].to_string();
    } else if fn_patt.is_match(content) {
        let cap = fn_patt.captures(content).unwrap();
        attr_set.name = cap["name"].to_string();
        attr_set.value = cap["value"].to_string();
    } else if attr_patt.is_match(content) {
        let cap = attr_patt.captures(content).unwrap();
        attr_set.name = cap["name"].to_string();
        attr_set.value = cap["value"].to_string();
    }
    Ok(attr_set)
}


