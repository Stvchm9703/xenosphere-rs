use crate::tokens::script_lang::variable_declartion_token::VariableDeclartionToken;
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallStatementToken {
    /// CFunctionCallStatementToken
    /// for the simple function call statement
    /// e.g. add(a, b);
    /// function_name : add
    /// function_args : [a, b]
    /// raw_content : "add(a, b);"
    pub function_name: String,
    pub function_args: Vec<VariableDeclartionToken>,
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for FunctionCallStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
