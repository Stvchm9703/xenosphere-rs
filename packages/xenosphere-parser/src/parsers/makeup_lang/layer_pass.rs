use crate::tokens::layer_lang::{
    LayerFuncArgSet, LayerFuncArgValue, LayerStackBlock, LayerStackElm,
};
use anyhow::Error;
use pest::iterators::{Pair, Pairs};

use crate::parsers::makeup_lang::Rule;
use crate::parsers::clang::Rule as ClangRule;

pub fn parse_layer_pass(pairs: Pairs<Rule>) -> Result<Vec<LayerStackElm>, Error> {
 
    

    println!("output {:#?}", layer_stack);

    Ok(layer_stack)
}