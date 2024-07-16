use anyhow::Error;
use pest::iterators::Pair;

use crate::parsers::makeup_lang::Rule;
use crate::tokens::{layer_lang::LayerPropertyElementValue, tensor::PseudoTensor};
pub fn parse_val_def_block(pair: Pair<Rule>) -> Result<(String, LayerPropertyElementValue), Error> {
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

pub fn parse_value_block(pair: Pair<Rule>) -> Result<LayerPropertyElementValue, Error> {
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

fn parse_tensor_block(
    income_data: LayerPropertyElementValue,
) -> Result<PseudoTensor<LayerPropertyElementValue>, Error> {
    let mut tensor_data = PseudoTensor {
        shape: vec![],
        data: vec![],
    };

    Ok(tensor_data)
}
