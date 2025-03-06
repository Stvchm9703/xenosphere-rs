use crate::tokens::layer_lang::{
    FuncArgSet, FuncArgValue, LayerStackBlock, LayerStackBranch, LayerStackElm,
};
use anyhow::Error;
use pest::iterators::{Pair, Pairs};

use crate::parsers::makeup_lang::Rule;

pub fn parse_layer_stack(pairs: Pairs<Rule>) -> Result<Vec<LayerStackElm>, Error> {
    // println!("parse_layer_stack");

    let mut layer_stack: Vec<LayerStackElm> = vec![];
    for (idx, pair) in pairs.enumerate() {
        // println!("rule {:?}", pair.as_rule().to_owned());
        // println!("value {:?}", pair.as_span().to_owned());
        // println!();

        match pair.as_rule() {
            Rule::stack_func_block => {
                let tmp = parse_stack_func_block(pair, idx)?;
                layer_stack.push(tmp);
            }
            // Rule::func_block => {
            //     let tmp = parse_stack_func_block(pair)?;
            //     layer_stack.push(tmp);
            // }
            Rule::branch_block => {
                let mut tmp = parse_branch_block(pair, idx)?;
                layer_stack.append(&mut tmp);
            }
            _ => {}
        }
    }
    // println!("output {:#?}", layer_stack);

    Ok(layer_stack)
}

fn parse_stack_func_block(pair: Pair<Rule>, org_id: usize) -> Result<LayerStackElm, Error> {
    // println!("parse_stack_func_block");
    let (pos, _) = pair.line_col();
    let mut func_block: LayerStackBlock = LayerStackBlock {
        id: org_id,
        func_name: "".to_string(),
        func_args: vec![],
        pos,
    };

    for inner_pair in pair.clone().into_inner() {
        // println!("rule {:?}", inner_pair.as_rule().to_owned());
        // println!("value {:?}", inner_pair.as_span().to_owned());
        // println!();

        match inner_pair.as_rule() {
            // Rule::naming => {
            //     func_block.func_name = inner_pair.as_str().to_string();
            // }
            Rule::imported_funciton_naming => {
                func_block.func_name = inner_pair.as_str().to_string();
            }
            Rule::func_arg_block => {
                // parse the func args
                func_block.func_args = extract_layer_func_arg(inner_pair);
            }
            _ => {}
        }
    }
    // println!();
    // println!("---");

    return Ok(LayerStackElm::Block(func_block));
}

fn extract_layer_func_arg(inner_pair: Pair<Rule>) -> Vec<FuncArgSet> {
    // println!("extract_layer_func_arg");

    let mut func_args: Vec<FuncArgSet> = vec![];
    for (pos, arg_pair) in inner_pair.clone().into_inner().enumerate() {
        // println!("rule {:?}", arg_pair.as_rule().to_owned());
        // println!("value {:?}", arg_pair.as_span().to_owned());
        // println!();

        match arg_pair.as_rule() {
            Rule::object_property => {
                let tmp = FuncArgSet {
                    name: format!("arg:{}", pos),
                    value: FuncArgValue::InputReference(
                        arg_pair.as_span().as_str().to_string(),
                    ),
                };
                func_args.push(tmp);
            }

            Rule::int_block => {
                let tmp = FuncArgSet {
                    name: format!("arg:{}", pos),
                    value: FuncArgValue::Int(arg_pair.as_str().parse::<i32>().unwrap()),
                };
                func_args.push(tmp);
            }

            Rule::float_block | Rule::num_block => {
                let tmp = FuncArgSet {
                    name: format!("arg:{}", pos),
                    value: FuncArgValue::Float(arg_pair.as_str().parse::<f32>().unwrap()),
                };
                func_args.push(tmp);
            }

            Rule::string_block => {
                let tmp = FuncArgSet {
                    name: format!("arg:{}", pos),
                    value: FuncArgValue::String(
                        arg_pair.as_str().to_string().replace("\"", ""),
                    ),
                };
                func_args.push(tmp);
            }

            Rule::func_name_arg_block => {
                let tmp_inner = arg_pair.clone().into_inner();
                let mut tmp = FuncArgSet {
                    name: "".to_string(),
                    value: FuncArgValue::String("".to_string()),
                };

                for tmp_inner_pair in tmp_inner {
                    match tmp_inner_pair.as_rule() {
                        Rule::naming => {
                            tmp.name = tmp_inner_pair.as_str().to_string();
                        }
                        Rule::int_block => {
                            tmp.value = FuncArgValue::Int(
                                tmp_inner_pair.as_str().parse::<i32>().unwrap(),
                            );
                        }
                        Rule::float_block | Rule::num_block => {
                            tmp.value = FuncArgValue::Float(
                                tmp_inner_pair.as_str().parse::<f32>().unwrap(),
                            );
                        }
                        Rule::string_block => {
                            tmp.value = FuncArgValue::String(
                                tmp_inner_pair.as_str().to_string().replace("\"", ""),
                            );
                        }
                        Rule::object_property => {
                            tmp.value = FuncArgValue::InputReference(
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

fn parse_branch_block(pair: Pair<Rule>, org_idx: usize) -> Result<Vec<LayerStackElm>, Error> {
    let mut branch_block: Vec<LayerStackElm> = vec![];
    // let (pos, _) = pair.line_col();
    // println!("branch_block");
    // println!("rule {:?}", pair.as_rule().to_owned());
    // println!("value {:?}", pair.as_span().to_owned());

    // branch_block = parse_layer_stack(pair.into_inner()).unwrap();
    // println!()
    for (idx, pair) in pair.into_inner().enumerate() {
        // println!("--rule {:?}", pair.as_rule().to_owned());
        // println!("--value {:?}", pair.as_span().to_owned());
        // println!();
        match pair.as_rule() {
            Rule::layer_stack_block => {
                let stack = parse_layer_stack(pair.into_inner()).unwrap();
                // layer_obj.stack = Some(stack);
                branch_block.push(LayerStackElm::Branch(LayerStackBranch {
                    id: org_idx,
                    pos: idx,
                    branch_id: org_idx,
                    stack_list: stack,
                }));
            }
            _ => {}
        }
    }

    return Ok(branch_block);
}
