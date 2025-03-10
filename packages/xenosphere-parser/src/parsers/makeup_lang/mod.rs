
use anyhow::Error;
use common::parse_attribute_set;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
// use syn::token::Type;
// use polars::prelude::*;

use crate::tokens::makeup_lang::{
    // self,
    LayerFileAttribute,
    LayerFileContent,
    LayerFileToken,
    LayerImportAttr,
    LayerObj,
    LayerPackageAttr,
    // LayerFuncArgSet,
    // LayerFuncArgValue,
    // LayerPropertyElement,
    // LayerPropertyElementSet, LayerPropertyElementValue, LayerStackBlock, LayerStackElm,
};

mod common;
mod layer_pass;
mod layer_property;
mod layer_stack;
mod value_define_block;

#[derive(Parser)]
#[grammar = "parsers/makeup_lang/syntax.pest"]
pub struct SyntexParser;

pub fn parse(input_str: &str) -> Result<LayerFileContent, Error> {
    let result_token = SyntexParser::parse(Rule::chunk, input_str);

    match result_token {
        Ok(pairs) => {
            let content_token: LayerFileContent = parse_content(pairs.to_owned())?;
            return Ok(content_token);
        }
        Err(e) => Err(Error::msg(format!("Error parsing input: {:?}", e))),
    }
}

fn parse_content(pairs: Pairs<Rule>) -> Result<LayerFileContent, Error> {
    let mut layer_file_tokens: LayerFileContent = vec![];
    for pair in pairs {
        // println!("parse-content");
        // println!(" rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        match pair.as_rule() {
            Rule::layer_def_block => {
                layer_file_tokens.push(LayerFileToken::Layer(parse_layer(pair.into_inner())?));
            }
            Rule::attr_def_block => {
                layer_file_tokens.extend(parse_attr_def_block(pair.into_inner())?);
            }
            _ => {
                layer_file_tokens.push(LayerFileToken::Unknown(pair.as_str().to_string()));
            }
        }
    }
    Ok(layer_file_tokens)
}

fn parse_layer(pairs: Pairs<Rule>) -> Result<LayerObj, Error> {
    // println!("parse_layer");
    let mut layer_obj = LayerObj {
        name: "".to_string(),
        property: None,
        pass: None,
        stack: None,
    };
    for pair in pairs {
        // println!("rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        // println!();
        match pair.as_rule() {
            Rule::naming => {
                layer_obj.name = pair.as_span().as_str().to_owned();
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
    // println!("out parse layer : {:#?}", layer_obj);

    Ok(layer_obj)
}

fn parse_attr_def_block(pairs: Pairs<Rule>) -> Result<Vec<LayerFileToken>, Error> {
    let inner: Vec<LayerFileToken> = parse_attribute_set(pairs)?
        .iter()
        .map(|x| {
            println!("x : {:?}", x);
            match x.name.as_str() {
                "import" => LayerFileToken::Attribute(LayerFileAttribute::Import(
                    LayerImportAttr::from_attribute_set(x),
                )),

                "export" => LayerFileToken::Attribute(LayerFileAttribute::Export(x.to_owned())),
                "package" => LayerFileToken::Attribute(LayerFileAttribute::Package(
                    LayerPackageAttr::from_attribute_set(x),
                )),
                _ => LayerFileToken::Attribute(LayerFileAttribute::Unknown(x.to_owned())),
            }
        })
        .collect();

    Ok(inner)
}
