use anyhow::Error;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
// use syn::token::Type;
// use polars::prelude::*;

use crate::tokens::layer_lang::{
    // self,
    LayerFileToken,
    LayerObj,
    // LayerFuncArgSet,
    // LayerFuncArgValue,
    // LayerPropertyElement,
    // LayerPropertyElementSet, LayerPropertyElementValue, LayerStackBlock, LayerStackElm,
};

mod layer_property;
mod layer_stack;
mod value_define_block;
mod layer_pass;
mod common;

#[derive(Parser)]
#[grammar = "parsers/makeup_lang/syntax.pest"]
pub struct SyntexParser;

pub fn parse(input_str: &str) -> std::result::Result<Pairs<Rule>, Error> {
    let result_token = SyntexParser::parse(Rule::chunk, input_str);

    match result_token {
        Ok(pairs) => {
            let content_token = parse_content(pairs.to_owned())?;
            println!("out parse_chunk {:#?}", content_token);
            return Ok(pairs);
        }
        Err(e) => Err(Error::msg(format!("Error parsing input: {:?}", e))),
    }
}

fn parse_content(pairs: Pairs<Rule>) -> Result<Vec<LayerFileToken>, Error> {
    let mut layer_file_tokens: Vec<LayerFileToken> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::layer_def_block => {
                let layer_obj = parse_layer(pair.into_inner())?;
                layer_file_tokens.push(LayerFileToken::Layer(layer_obj));
            }
            Rule::attr_def_block => {
                // parse_attribute(pair.into_inner())?;
            }
            _ => {
                layer_file_tokens.push(LayerFileToken::Unknown(pair.as_str().to_string()));
            }
        }
    }
    Ok(layer_file_tokens)
}

fn parse_layer(pairs: Pairs<Rule>) -> Result<LayerObj, Error> {
    println!("parse_layer");
    let mut layer_obj = LayerObj {
        name: "".to_string(),
        property: None,
        pass: None,
        stack: None,
    };
    for pair in pairs {
        match pair.as_rule() {
            Rule::naming => {
                layer_obj.name = pair.as_str().to_string();
            }
            Rule::layer_property_block => {
                layer_obj.property = Some(layer_property::parse_layer_property(pair.into_inner())?);
            }
            Rule::layer_pass_block => {
                layer_obj.pass = Some(layer_pass::parse_layer_pass(pair.into_inner())?);
            }
            Rule::layer_stack_block => {
                layer_obj.stack = Some(layer_stack::parse_layer_stack(pair.into_inner())?);
            }
            _ => {}
        }
    }
    Ok(layer_obj)
}
