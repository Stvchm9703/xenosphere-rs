use anyhow::Error;
use pest::iterators::{Pair, Pairs};
// use pest::Parser;
// use pest_derive::Parser;
// use syn::token::Type;
// use polars::prelude::*;

use crate::tokens::layer_lang::{
    LayerPropertyElement, LayerPropertyElementSet, LayerPropertyElementValue,
};

use crate::parsers::makeup_lang::{Rule, value_define_block::parse_val_def_block};

pub fn parse_layer_property(pairs: Pairs<Rule>) -> Result<Vec<LayerPropertyElement>, Error> {
    let mut layer_properties: Vec<LayerPropertyElement> = vec![];
    println!("in parse_layer_property");
    for pair in pairs {
        // println!("parse_layer_property rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        match pair.as_rule() {
            Rule::layer_property_elem => {
                let tmp = parse_layer_property_element_set(pair)?;
                layer_properties.push(tmp);
            }
            // Rule::layer_property_elem_dim => {
            //     let tmp = parse_layer_property_elem_dim(pair)?;
            //     layer_properties.push(tmp);
            // }
            _ => {
                let tmp = LayerPropertyElement::Unknown(pair.as_span().as_str().to_string());
                layer_properties.push(tmp);
            }
        }
    }
    // println!("out parse_layer_property   {:#?}", layer_properties);
    Ok(layer_properties)
}

pub fn parse_layer_property_element_set(pair: Pair<Rule>) -> Result<LayerPropertyElement, Error> {
    let mut layer_property_element_set = LayerPropertyElementSet {
        name: "".to_string(),
        value: LayerPropertyElementValue::Int(0),
    };

    // println!("parse_layer_property_element_set {:#?}", pair);

    for pair in pair.clone().into_inner() {
        // parse the val_def_block
        let (name, value) = parse_val_def_block(pair)?;
        layer_property_element_set.name = name;
        layer_property_element_set.value = value;
    }

    // extract the prefix, either "static" or "in" or "out"
    let prefix = pair.as_str().to_string();
    if prefix.contains("static") {
        return Ok(LayerPropertyElement::Static(layer_property_element_set));
    } else if prefix.contains("in") {
        return Ok(LayerPropertyElement::In(layer_property_element_set));
    } else if prefix.contains("out") {
        return Ok(LayerPropertyElement::Out(layer_property_element_set));
    }
    Ok(LayerPropertyElement::Unknown(
        pair.as_span().as_str().to_string(),
    ))
}
