use crate::tokens::makeup_lang::LayerPassScript;
use anyhow::Error;
use pest::iterators::Pairs;

use crate::parsers::makeup_lang::{Rule, common::parse_attribute_set};

pub fn parse_layer_pass(pairs: Pairs<Rule>) -> Result<Vec<LayerPassScript>, Error> {
    let mut layer_passes: Vec<LayerPassScript> = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::pass_impl_block => {
                layer_passes.push(parse_layer_pass_block(pair.into_inner())?);
            }
            _ => {}
        }
    }

    Ok(layer_passes)
}

pub fn parse_layer_pass_block(pairs: Pairs<Rule>) -> Result<LayerPassScript, Error> {
    let mut pass_set = LayerPassScript::default();

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
    // println!("pairs {:?}", pairs);
    let attr_sets = parse_attribute_set(pairs)?;

    // println!("attr_sets {:?}", attr_sets);

    for attr_set in attr_sets {
        // println!("attr_set {:?}", attr_set);

        match attr_set.name.as_str() {
            "target_platform" | "target" => {
                // attr_set.value.
                pass_set.target_platform = attr_set.value.get("default").unwrap().to_owned();
            }
            "syntex" => {
                pass_set.script_syntex = Some(attr_set.value.get("default").unwrap().to_owned());
            }
            "filename" => {
                pass_set.script_filename =
                    Some(attr_set.value.get(&attr_set.name).unwrap().to_owned());
            }
            "transpiler" => {
                pass_set.transpiler = Some(attr_set.value.get(&attr_set.name).unwrap().to_owned());
            }
            "skip_transpile" => {
                pass_set.allow_transpile = false;
            }
            "allow_transpile" => {
                let tmp = attr_set.value.get(&attr_set.name).unwrap();
                pass_set.allow_transpile = if tmp.contains("true")
                    || tmp.contains("True")
                    || tmp.contains("TRUE")
                    || tmp.is_empty()
                {
                    true
                } else {
                    false
                };
            }
            "overwrite" => {
                let is_ow = attr_set.value.get(&attr_set.name).unwrap();

                pass_set.is_overwrite = if is_ow.contains("true")
                    || is_ow.contains("True")
                    || is_ow.contains("TRUE")
                    || is_ow.is_empty()
                {
                    true
                } else {
                    false
                };
            }
            _ => {}
        }
    }

    Ok(())
}
