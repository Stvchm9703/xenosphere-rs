use crate::tokens::layer_lang::{
    AttributeSet, LayerFuncArgSet, LayerFuncArgValue, LayerPass, LayerPassScript, LayerStackBlock,
    LayerStackElm,
};
use anyhow::Error;
use pest::iterators::{Pair, Pairs};

use crate::parsers::makeup_lang::{common::parse_attribute_set, Rule};

pub fn parse_layer_pass(pairs: Pairs<Rule>) -> Result<Vec<LayerPassScript>, Error> {
    let mut layer_passes: Vec<LayerPassScript> = vec![];

    // println!("output {:#?}", layer_stack);

    for pair in pairs {
        // println!(
        //     "parse_layer_property rule => {:?}",
        //     pair.as_rule().to_owned()
        // );
        // println!("value => {:?}", pair.as_span().to_owned());
        match pair.as_rule() {
            Rule::pass_impl_block => {
                layer_passes.push(parse_layer_pass_block(pair.into_inner())?);
            }
            _ => {}
        }
    }
    println!("parse_layer_pass out => {:#?}", layer_passes);

    Ok(layer_passes)
}

pub fn parse_layer_pass_block(pairs: Pairs<Rule>) -> Result<LayerPassScript, Error> {
    let mut pass_set = LayerPassScript {
        raw_content: String::new(),
        script_filename: None,
        script_syntex: None,
        target_platform: "".to_owned(),
        transpiler: None,
    };

    for pair in pairs {
        // println!("parse_layer_pass_block rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        match pair.as_rule() {

            Rule::attr_def_block => {
                parse_layer_pass_impl_attr_block(pair.into_inner(), &mut pass_set)?;
            }
            Rule::pass_impl_content_block => {
                pass_set.raw_content = pair.as_str().to_owned();
            }
            _ => {}
        }
    }

    Ok(pass_set)
}

pub fn parse_layer_pass_impl_attr_block(
    pairs: Pairs<Rule>,
    pass_set: &mut LayerPassScript,
) -> Result<(), Error> {
    let attr_sets = parse_attribute_set(pairs)?;

    for attr_set in attr_sets {
        match attr_set.name.as_str() {
            "target" => {
                pass_set.target_platform = attr_set.value;
            }
            "syntex" => {
                pass_set.script_syntex = Some(attr_set.value);
            }
            "filename" => {
                pass_set.script_filename = Some(attr_set.value);
            }
            "transpiler" => {
                pass_set.transpiler = Some(attr_set.value);
            }
            _ => {}
        }
    }

    Ok(())
}
