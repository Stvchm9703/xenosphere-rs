use anyhow::Error;
use common::parse_attribute_set_content;
use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
// use syn::token::Type;
// use polars::prelude::*;

use crate::tokens::makeup_lang::{
    // self,
    FileAttribute,
    FileImportAttr,
    FilePackageAttr,
    // LayerFuncArgSet,
    // LayerFuncArgValue,
    // LayerPropertyElement,
    // LayerPropertyElementSet, LayerPropertyElementValue, LayerStackBlock, LayerStackElm,
    LayerObj,
    XeslFileContent,
    XeslFileToken,
};

mod common;
mod layer_pass;
mod layer_property;
mod layer_stack;
mod value_define_block;

#[derive(Parser)]
#[grammar = "parsers/makeup_lang/syntax.pest"]
pub struct SyntexParser;

pub fn parse(input_str: &str) -> Result<XeslFileContent, Error> {
    let result_token = SyntexParser::parse(Rule::chunk, input_str);

    match result_token {
        Ok(pairs) => {
            let content_token: XeslFileContent = parse_content(pairs.to_owned())?;
            return Ok(content_token);
        }
        Err(e) => Err(Error::msg(format!("Error parsing input: {:?}", e))),
    }
}

fn parse_content(pairs: Pairs<Rule>) -> Result<XeslFileContent, Error> {
    let mut layer_file_tokens: Vec<XeslFileToken> = vec![];
    for pair in pairs {
        // println!("parse-content");
        // println!(" rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        match pair.as_rule() {
            Rule::layer_def_block => {
                layer_file_tokens.push(XeslFileToken::Layer(parse_layer(pair.into_inner())?));
            }
            Rule::attr_def_block => {
                layer_file_tokens.push(parse_attr_def_block(pair.into_inner())?);
            }
            _ => {
                let tmp = pair.as_str().to_string();
                // println!("parse_content unresolved");
                // println!("{:?}", pair);
                if tmp.is_empty() == false {
                    layer_file_tokens.push(XeslFileToken::Unknown(tmp));
                }
            }
        }
    }

    Ok(XeslFileContent {
        content: layer_file_tokens,
        ..Default::default()
    })
}

fn parse_layer(pairs: Pairs<Rule>) -> Result<LayerObj, Error> {
    // println!("parse_layer");
    let mut layer_obj = LayerObj::default();
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
            _ => (),
        }
    }
    // println!("out parse layer : {:#?}", layer_obj);

    Ok(layer_obj)
}

fn parse_attr_def_block(pairs: Pairs<Rule>) -> Result<XeslFileToken, Error> {
    // let inner: Vec<XeslFileToken> = parse_attribute_set(pairs.to_owned())?
    //     .iter()
    //     .map(|x| {
    //         // println!("x : {:?}", x);
    //         match x.name.as_str() {
    //             "import" => XeslFileToken::Attribute(FileAttribute::Import(
    //                 FileImportAttr::from_attribute_set(x),
    //             )),

    //             "export" => XeslFileToken::Attribute(FileAttribute::Export(x.to_owned())),
    //             "package" => XeslFileToken::Attribute(FileAttribute::Package(
    //                 FilePackageAttr::from_attribute_set(x),
    //             )),
    //             _ => XeslFileToken::Attribute(FileAttribute::Unknown(x.to_owned())),
    //         }
    //     })
    //     .collect();

    let pair_item = pairs.to_owned().into_iter().next().unwrap();
    let file_token = parse_attribute_set_content(pair_item)?;
    let inner = match file_token.name.as_str() {
        "import" => XeslFileToken::Attribute(FileAttribute::Import(
            FileImportAttr::from_attribute_set(&file_token),
        )),
        "export" => XeslFileToken::Attribute(FileAttribute::Export(file_token.to_owned())),
        "package" => XeslFileToken::Attribute(FileAttribute::Package(
            FilePackageAttr::from_attribute_set(&file_token),
        )),
        _ => XeslFileToken::Attribute(FileAttribute::Unknown(file_token.to_owned())),
    };
    Ok(inner)
}
