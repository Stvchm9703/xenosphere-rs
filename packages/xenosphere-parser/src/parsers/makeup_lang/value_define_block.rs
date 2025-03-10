use std::i32;

use anyhow::{Error, Ok};
use pest::iterators::Pair;
// use syn::parse;

use crate::parsers::makeup_lang::Rule;
use crate::tokens::makeup_lang::{
    FuncArgSet, FuncArgValue, FuncCallSet, LayerPropertyElementSet, LayerPropertyElementValue,
};
use crate::tokens::tensor::PseudoTensor;

pub fn parse_val_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    // println!("rule: {:?}", pair.as_rule());

    for pair_item in pair.clone().into_inner() {
        // println!("//rule: {:?}", pair_item.as_rule());
        match pair_item.as_rule() {
            Rule::int_def_block => {
                return Ok(parse_int_def_block(pair_item)?);
            }
            Rule::float_def_block => {
                return Ok(parse_float_def_block(pair_item)?);
            }
            Rule::string_def_block => {
                return Ok(parse_string_def_block(pair_item)?);
            }

            Rule::func_def_block => {
                return Ok(parse_func_def_block(pair_item)?);
            }

            Rule::array_def_block => {
                return Ok(parse_array_def_block(pair_item)?);
            }

            Rule::tensor_def_block => {}
            _ => {}
        };
        // println!("//rule: {:?}", pair_item.as_rule());
    }

    // Err(Error::msg("unsolved"))
    Ok(LayerPropertyElementSet {
        name: "emp".to_owned(),
        value: LayerPropertyElementValue::None,
    })
}

fn parse_int_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    let mut name = "".to_string();
    // let mut shape: Vec<i32> = vec![];
    let mut value = LayerPropertyElementValue::Int(0);

    for pair_item in pair.clone().into_inner() {
        // println!("rule: {:?}", pair_item.as_rule());
        // println!("value: {:?}" ,pair_item.as_span());

        match pair_item.as_rule() {
            Rule::naming => {
                name = pair_item.as_str().to_string();
            }
            Rule::int_block => {
                value = int_block(&pair_item);
            }
            _ => {}
        }
    }
    return Ok(LayerPropertyElementSet { name, value });
}

fn parse_float_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    let mut name = "".to_string();
    // let mut shape: Vec<i32> = vec![];
    let mut value = LayerPropertyElementValue::Float(0f32);

    for pair_item in pair.clone().into_inner() {
        // println!("rule: {:?}", pair_item.as_rule());
        // println!("value: {:?}" ,pair_item.as_span());

        match pair_item.as_rule() {
            Rule::naming => {
                name = pair_item.as_str().to_string();
            }
            Rule::num_block | Rule::float_block => {
                value = float_block(&pair_item);
            }

            _ => {}
        }
    }
    return Ok(LayerPropertyElementSet { name, value });
}

fn parse_string_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    let mut name = "".to_string();
    // let mut shape: Vec<i32> = vec![];
    let mut value = LayerPropertyElementValue::String("".to_owned());

    for pair_item in pair.clone().into_inner() {
        // println!("rule: {:?}", pair_item.as_rule());
        // println!("value: {:?}" ,pair_item.as_span());

        match pair_item.as_rule() {
            Rule::naming => {
                name = pair_item.as_str().to_string();
            }
            Rule::string_block => {
                value = string_block(&pair_item);
            }

            _ => {}
        }
    }
    return Ok(LayerPropertyElementSet { name, value });
}

fn parse_func_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    let mut name = "".to_string();

    let mut func_set = FuncCallSet {
        func_name: String::new(),
        func_arg: None,
    };

    for pair_item in pair.clone().into_inner() {
        match pair_item.as_rule() {
            Rule::naming => {
                // println!("name : {:?}", pair_item.as_str().to_string());
                if name.is_empty() {
                    name = pair_item.as_str().to_string();
                } else {
                    func_set.func_name = pair_item.as_str().to_string();
                }
            }

            Rule::func_arg_block => {
                let mut arg_list: Vec<FuncArgSet> = Vec::new();
                for (arg_pos, arg_pair) in pair_item.into_inner().enumerate() {
                    if let Some(arg) = func_arg_block(&arg_pair, arg_pos) {
                        arg_list.push(arg);
                    }
                }
                func_set.func_arg = Some(arg_list);
            }

            _ => {}
        }
    }
    // println!("func-set {:?}", func_set);

    return Ok(LayerPropertyElementSet {
        name,
        value: LayerPropertyElementValue::Func(func_set),
    });
}

fn parse_array_def_block(pair: Pair<Rule>) -> Result<LayerPropertyElementSet, Error> {
    let mut name = "".to_string();
    // let mut shape: Vec<i32> = vec![];
    let mut value = LayerPropertyElementValue::Array(vec![]);

    for pair_item in pair.clone().into_inner() {
        match pair_item.as_rule() {
            Rule::naming => {
                name = pair_item.as_str().to_string();
            }
            Rule::array_block => {
                value = array_block(&pair_item);
            }

            _ => {}
        }
    }
    return Ok(LayerPropertyElementSet { name, value });
}

pub fn parse_value_block(pair: Pair<Rule>) -> Result<LayerPropertyElementValue, Error> {
    // println!("parse_value_block {:?}", pair);
    match pair.as_rule() {
        Rule::int_block => Ok(int_block(&pair)),
        Rule::num_block | Rule::float_block => Ok(float_block(&pair)),
        Rule::string_block => Ok(string_block(&pair)),
        // Rule::func_block => {
        //     // let tmp_value = pair.as_str().trim();
        //     // return Ok(LayerPropertyElementValue::Func());
        // }
        Rule::array_block => Ok(array_block(&pair)),

        _ => Ok(LayerPropertyElementValue::None),
    }
}

fn int_block(pair: &Pair<'_, Rule>) -> LayerPropertyElementValue {
    let tmp_value = pair.as_str().trim();
    return LayerPropertyElementValue::Int(tmp_value.parse::<i32>().unwrap());
}

fn float_block(pair: &Pair<'_, Rule>) -> LayerPropertyElementValue {
    let tmp_value = pair.as_str().trim();
    return LayerPropertyElementValue::Float(tmp_value.parse::<f32>().unwrap());
}

fn string_block(pair: &Pair<'_, Rule>) -> LayerPropertyElementValue {
    let tmp_value = pair.as_str().trim();
    return LayerPropertyElementValue::String(tmp_value.to_string());
}

fn array_block(pair: &Pair<'_, Rule>) -> LayerPropertyElementValue {
    let tmp_sub_values = pair
        .clone()
        .into_inner()
        .map(|x| parse_value_block(x).unwrap())
        .collect::<Vec<LayerPropertyElementValue>>();
    return LayerPropertyElementValue::Array(tmp_sub_values);
}

fn func_arg_block(pair: &Pair<'_, Rule>, id: usize) -> Option<FuncArgSet> {
    match pair.as_rule() {
        Rule::object_property => Some(FuncArgSet {
            name: format!("args:{}", id),
            value: FuncArgValue::InputReference(pair.as_str().to_owned()),
        }),
        Rule::func_name_arg_block => {
            let mut tmp = FuncArgSet {
                name: String::new(),
                value: FuncArgValue::None,
            };

            for inner_pair in pair.clone().into_inner() {
                match inner_pair.as_rule() {
                    Rule::naming => {
                        tmp.name = inner_pair.as_str().to_owned();
                    }
                    Rule::int_block => {
                        tmp.value = FuncArgValue::Int(inner_pair.as_str().parse::<i32>().unwrap());
                    }
                    Rule::float_block | Rule::num_block => {
                        tmp.value =
                            FuncArgValue::Float(inner_pair.as_str().parse::<f32>().unwrap());
                    }
                    Rule::string_block => {
                        tmp.value =
                            FuncArgValue::String(inner_pair.as_str().to_string().replace("\"", ""));
                    }
                    Rule::object_property => {
                        tmp.value =
                            FuncArgValue::InputReference(inner_pair.as_span().as_str().to_string());
                    }
                    _ => {}
                }
            }

            Some(tmp)
        }
        _ => None,
    }
}

fn parse_tensor_block(
    income_data: LayerPropertyElementValue,
) -> Result<PseudoTensor<LayerPropertyElementValue>, Error> {
    let tensor_data = PseudoTensor {
        shape: vec![],
        data: vec![],
    };

    Ok(tensor_data)
}
