use anyhow::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use syn::token::Type;
// use polars::prelude::*;

use crate::tokens::layer_lang::{
    self, LayerFileToken, LayerFuncArgSet, LayerFuncArgValue, LayerObj, LayerPropertyElement,
    LayerPropertyElementSet, LayerPropertyElementValue, LayerStackBlock, LayerStackElm,
};

#[derive(Parser)]
#[grammar = "parsers/syntax.pest"]
pub struct SyntexParser;

pub fn parse(input_str: &str) -> std::result::Result<Pairs<Rule>, Error> {
    let result_token = SyntexParser::parse(Rule::chunk, input_str);

    match result_token {
        Ok(pairs) => {
            let content_token = parse_chunk(pairs.to_owned())?;
            // println!("out parse_chunk {:#?}", content_token);
            return Ok(pairs);
        }
        Err(e) => Err(Error::msg(format!("Error parsing input: {:?}", e))),
    }
}

fn parse_chunk(pairs: Pairs<Rule>) -> Result<Vec<LayerFileToken>, Error> {
    println!("in parse_chunk func");

    let mut layer_file_tokens: Vec<LayerFileToken> = vec![];
    for pair in pairs {
        // println!("parse_chunk pairs {:?}", pair.as_rule());
        match pair.as_rule() {
            Rule::content => {
                let content_tokens = parse_content(pair.into_inner())?;
                layer_file_tokens.extend(content_tokens);
            }

            _ => {
                layer_file_tokens.push(LayerFileToken::Unknown(pair.as_str().to_string()));
            }
        }
    }
    Ok(layer_file_tokens)
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
                layer_obj.property = Some(parse_layer_property(pair.into_inner())?);
            }
            // Rule::layer_pass_block => {
            //     layer_obj.pass = Some(parse_layer_pass(pair.into_inner())?);
            // }
            Rule::layer_stack_block => {
                layer_obj.stack = Some(parse_layer_stack(pair.into_inner())?);
            }
            _ => {}
        }
    }
    Ok(layer_obj)
}

fn parse_layer_property(pairs: Pairs<Rule>) -> Result<Vec<LayerPropertyElement>, Error> {
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

fn parse_layer_property_element_set(pair: Pair<Rule>) -> Result<LayerPropertyElement, Error> {
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

fn parse_layer_stack(pairs: Pairs<Rule>) -> Result<Vec<LayerStackElm>, Error> {
    // println!("parse_layer_stack");

    let mut layer_stack: Vec<LayerStackElm> = vec![];
    for pair in pairs {
        // println!("elm {:#?}", pair);
        // println!("parse_layer_stack rule {:?}", pair.as_rule().to_owned());

        match pair.as_rule() {

            Rule::func_block => {
                // let tmp = parse_layer_stack_element(pair)?;
                // layer_stack.push(tmp);
                // let elm = parse_value_block(pair)?;
                println!("elm {:#?}", pair);
                let tmp = parse_stack_func_block(pair)?;
                layer_stack.push(tmp);
            }
            Rule::branch_block => {
                // let tmp = parse_layer_stack_element(pair)?;
                // layer_stack.push(tmp);
                // let elm = parse_value_block(pair)?;
                // println!("elm {:#?}", elm);
            }
            _ => {}
        }

    }
    println!("output {:#?}", layer_stack);

    Ok(layer_stack)
}

fn parse_val_def_block(pair: Pair<Rule>) -> Result<(String, LayerPropertyElementValue), Error> {
    let mut name = "".to_string();
    let mut shape: Vec<i32> = vec![];
    let mut value = LayerPropertyElementValue::None;

    for pair_item in pair.clone().into_inner() {
        match pair_item.as_rule() {
            Rule::naming => {
                name = pair_item.as_str().to_string();
            }
            Rule::shape_block => {
                let tmp_shape = pair_item.as_str().trim();
                shape = tmp_shape[1..tmp_shape.len() - 1]
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
            }
            Rule::int_block
            | Rule::float_block
            | Rule::num_block
            | Rule::string_block
            | Rule::func_block => {
                value = parse_value_block(pair_item)?;
            }
            Rule::array_block => {
                value = parse_value_block(pair_item)?;
                // if pair.clone().as_rule() == Rule::tensor_def_block {
                // }
            }
            _ => {}
        }
    }

    Ok((name, value))
}

fn parse_value_block(pair: Pair<Rule>) -> Result<LayerPropertyElementValue, Error> {
    print!("parse_value_block {:#?}", pair);
    match pair.as_rule() {
        Rule::int_block => {
            let tmp_value = pair.as_str().trim();
            return Ok(LayerPropertyElementValue::Int(
                tmp_value.parse::<i32>().unwrap(),
            ));
        }
        Rule::num_block | Rule::float_block => {
            let tmp_value = pair.as_str().trim();
            return Ok(LayerPropertyElementValue::Float(
                tmp_value.parse::<f32>().unwrap(),
            ));
        }
        Rule::string_block => {
            let tmp_value = pair.as_str().trim();
            return Ok(LayerPropertyElementValue::String(tmp_value.to_string()));
        }
        // Rule::func_block => {
        //     // let tmp_value = pair.as_str().trim();
        //     // return Ok(LayerPropertyElementValue::Func());
        // }
        Rule::array_block => {
            let tmp_sub_values = pair
                .clone()
                .into_inner()
                .map(|x| parse_value_block(x).unwrap())
                .collect::<Vec<LayerPropertyElementValue>>();
            return Ok(LayerPropertyElementValue::Array(tmp_sub_values));
        }
        _ => Ok(LayerPropertyElementValue::None),
    }
}

use crate::tokens::tensor::{PseudoTensor, PseudoTensorData};

pub fn parse_tensor_block(
    income_data: LayerPropertyElementValue,
) -> Result<PseudoTensor<LayerPropertyElementValue>, Error> {
    let mut tensor_data = PseudoTensor {
        shape: vec![],
        data: vec![],
    };

    Ok(tensor_data)
}

fn parse_stack_func_block(pair: Pair<Rule>) -> Result<LayerStackElm, Error> {
    let mut func_block: LayerStackBlock = LayerStackBlock {
        func_name: "".to_string(),
        func_args: vec![],
    };

    for inner_pair in pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::naming => {
                func_block.func_name = inner_pair.as_str().to_string();
            }
            Rule::func_arg_block => {
                // parse the func args
                func_block.func_args =  extract_layer_func_arg(inner_pair);
            }
            _ => {}
        }
    }

    return Ok(LayerStackElm::Block(func_block));
}

fn extract_layer_func_arg(inner_pair: Pair<Rule>) -> Vec<LayerFuncArgSet>{
    let mut func_args: Vec<LayerFuncArgSet> = vec![];
    for (pos, arg_pair) in inner_pair.clone().into_inner().enumerate() {
        match arg_pair.as_rule() {
            Rule::object_property => {
                let tmp = LayerFuncArgSet {
                    name: format!("arg:{}", pos),
                    value: LayerFuncArgValue::InputReference(
                        arg_pair.as_span().as_str().to_string(),
                    ),
                };
                func_args.push(tmp);
            }

            Rule::int_block => {
                let tmp = LayerFuncArgSet {
                    name: format!("arg:{}", pos),
                    value: LayerFuncArgValue::Int(
                        arg_pair.as_str().parse::<i32>().unwrap(),
                    ),
                };
                func_args.push(tmp);
            }

             Rule::float_block | Rule::num_block => {
                let tmp = LayerFuncArgSet {
                    name: format!("arg:{}", pos),
                    value: LayerFuncArgValue::Float(
                        arg_pair.as_str().parse::<f32>().unwrap(),
                    ),
                };
                func_args.push(tmp);
            }

            Rule::string_block => {
                let tmp = LayerFuncArgSet {
                    name: format!("arg:{}", pos),
                    value: LayerFuncArgValue::String(
                        arg_pair.as_str().to_string().replace("\"", ""),
                    ),
                };
                func_args.push(tmp);
            }

            Rule::func_name_arg_block => {
                let tmp_inner = arg_pair.clone().into_inner();
                let mut tmp = LayerFuncArgSet {
                    name: "".to_string(),
                    value: LayerFuncArgValue::String("".to_string()),
                };

                for tmp_inner_pair in tmp_inner {
                    match tmp_inner_pair.as_rule() {
                        Rule::naming => {
                            tmp.name = tmp_inner_pair.as_str().to_string();
                        }
                        Rule::int_block => {
                            tmp.value = LayerFuncArgValue::Int(
                                tmp_inner_pair.as_str().parse::<i32>().unwrap(),
                            );
                        }
                        Rule::float_block | Rule::num_block => {
                            tmp.value = LayerFuncArgValue::Float(
                                tmp_inner_pair.as_str().parse::<f32>().unwrap(),
                            );
                        }
                        Rule::string_block => {
                            tmp.value = LayerFuncArgValue::String(
                                tmp_inner_pair.as_str().to_string().replace("\"", ""),
                            );
                        }
                        Rule::object_property => {
                            tmp.value = LayerFuncArgValue::InputReference(
                                tmp_inner_pair.as_span().as_str().to_string(),
                            );
                        }
                        _ => {}
                    }
                }

                func_args.push(tmp);
            }

            _ => {}
        }
    }

    func_args
}

fn parse_branch_block(pair: Pair<Rule>) -> Result<LayerStackElm, Error> {
    let mut branch_block: Vec<LayerStackElm> = vec![];

    return Ok(LayerStackElm::Branch(branch_block));
}
