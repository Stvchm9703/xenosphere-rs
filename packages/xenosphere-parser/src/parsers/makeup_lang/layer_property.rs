use anyhow::{Error, Ok};
use pest::iterators::{Pair, Pairs};

use crate::tokens::makeup_lang::{LayerPropertyElementSet, LayerPropertyPrefix};

use crate::parsers::makeup_lang::{value_define_block::parse_val_def_block, Rule};

pub fn parse_layer_property(pairs: Pairs<Rule>) -> Result<Vec<LayerPropertyElementSet>, Error> {
    let mut layer_properties: Vec<LayerPropertyElementSet> = vec![];
    // println!("in parse_layer_property");
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
                layer_properties.push(LayerPropertyElementSet::new_unresolved(
                    pair.as_span().as_str().to_owned(),
                ));
            }
        }
    }
    // println!("out parse_layer_property   {:#?}", layer_properties);
    Ok(layer_properties)
}

pub fn parse_layer_property_element_set(
    pair: Pair<Rule>,
) -> Result<LayerPropertyElementSet, Error> {
    // println!("parse_layer_property_element_set {:#?}", pair.as_rule());
    let inner = pair.clone().into_inner().next().unwrap();
    let mut layer_property_element_set: LayerPropertyElementSet = parse_val_def_block(inner)?;
    // println!("result : {:?}", layer_property_element_set);
    // println!("===");

    // extract the prefix, either "static" or "in" or "out"
    let prefix = pair.as_str().to_string();
    if prefix.contains("static") {
        // return Ok(LayerPropertyPrefix::Static(layer_property_element_set));
        layer_property_element_set.prefix = LayerPropertyPrefix::Static;
    } else if prefix.contains("in") {
        // return Ok(LayerPropertyPrefix::In(layer_property_element_set));
        layer_property_element_set.prefix = LayerPropertyPrefix::In;
    } else if prefix.contains("out") {
        // return Ok(LayerPropertyPrefix::Out(layer_property_element_set));
        layer_property_element_set.prefix = LayerPropertyPrefix::Out;
    }
    Ok(layer_property_element_set)
}
