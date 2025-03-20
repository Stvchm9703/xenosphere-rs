use anyhow::Error;
use pest::iterators::{Pair, Pairs};

use crate::parsers::makeup_lang::Rule;
use crate::tokens::makeup_lang::AttributeSet;

/// main to resolve the attribute set `attr_set`
pub fn parse_attribute_set(pairs: Pairs<Rule>) -> Result<Vec<AttributeSet>, Error> {
    let mut attrs: Vec<AttributeSet> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::attr_set => {
                attrs.push(parse_attribute_set_content(pair)?);
            }
            _ => {}
        }
    }
    return Ok(attrs);
}

pub fn parse_attribute_set_content(pair: Pair<Rule>) -> Result<AttributeSet, Error> {
    // let content = pair.as_span().as_str();

    // println!("parse_attribute_set_content :\n {:#?}", pair);

    let mut attr_set = AttributeSet::default();

    // let func_block =
    //     // if pair.as_span().as_str() == "attr_set" {
    //     pair.into_inner().into_iter().nth(0).unwrap();
    // // } else {
    // //     pair
    // };

    // println!(" func_block: {:?}", func_block);

    for (idx, inner_pair) in pair.into_inner().enumerate() {
        // println!("inner_pair : {:?}", inner_pair);
        match inner_pair.as_rule() {
            Rule::naming => {
                attr_set.name = inner_pair.as_span().as_str().to_string();
            }
            Rule::object_property
            | Rule::string_block
            | Rule::float_block
            | Rule::num_block
            | Rule::int_block => {
                attr_set.value.insert(
                    "default".to_string(),
                    inner_pair.as_span().as_str().to_string().replace("\"", ""),
                );
            }
            // Rule::func_arg_block => {
            //     for arg_pair in inner_pair.clone().into_inner() {
            //         // let mut tmp_name = String::new();
            //         let mut tmp_value = String::new();
            //         match arg_pair.as_rule() {
            //             // Rule::naming => {
            //             //     attr_set.name = arg_pair.as_span().as_str().to_string();
            //             // }
            //             Rule::object_property
            //             | Rule::string_block
            //             | Rule::float_block
            //             | Rule::num_block
            //             | Rule::int_block => tmp_value = arg_pair.as_str().to_owned(),

            //             Rule::func_name_arg_block => {
            //                 let mut tmp_name = String::new();
            //                 for named_arg_pair in arg_pair.into_inner() {
            //                     // println!("func_name_arg_block : {:?}", named_arg_pair);
            //                     match named_arg_pair.as_rule() {
            //                         Rule::naming => {
            //                             tmp_name = named_arg_pair.as_span().as_str().to_string()
            //                         }
            //                         Rule::object_property
            //                         | Rule::string_block
            //                         | Rule::float_block
            //                         | Rule::num_block
            //                         | Rule::int_block => {
            //                             attr_set.value.insert(
            //                                 tmp_name.to_owned(),
            //                                 named_arg_pair.as_span().as_str().to_string(),
            //                             );
            //                         }
            //                         _ => (),
            //                     }
            //                 }
            //             }

            //             _ => {}
            //         }
            //         if tmp_value.is_empty() == false {
            //             attr_set.value.insert(format!("arg:{}", idx), tmp_value);
            //         }
            //     }
            // }
            Rule::func_block => {
                for func_block_pair in inner_pair.into_inner() {
                    let mut tmp_value = String::new();

                    match func_block_pair.as_rule() {
                        Rule::naming => {
                            attr_set.name = func_block_pair.as_span().as_str().to_string();
                        }
                        Rule::func_name_arg_block => {
                            let mut tmp_name = String::new();
                            for named_arg_pair in func_block_pair.into_inner() {
                                // println!("func_name_arg_block : {:?}", named_arg_pair);
                                match named_arg_pair.as_rule() {
                                    Rule::naming => {
                                        tmp_name = named_arg_pair.as_span().as_str().to_string()
                                    }
                                    Rule::object_property
                                    | Rule::string_block
                                    | Rule::float_block
                                    | Rule::num_block
                                    | Rule::int_block => {
                                        attr_set.value.insert(
                                            tmp_name.to_owned(),
                                            named_arg_pair.as_span().as_str().to_string(),
                                        );
                                    }
                                    _ => (),
                                }
                            }
                        }

                        _ => {}
                    }
                    if tmp_value.is_empty() == false {
                        attr_set.value.insert(format!("arg:{}", idx), tmp_value);
                    }
                }
            }
            _ => (),
        }
    }

    // println!("attr_set: {:?} \n", attr_set);

    Ok(attr_set)
}
