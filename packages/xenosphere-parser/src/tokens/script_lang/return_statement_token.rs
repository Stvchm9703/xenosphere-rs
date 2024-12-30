use crate::tokens::script_lang::value_operation_token::ValueOperationToken;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnStatementToken {
    /// CFunctionReturnStatementToken
    /// for the simple function return statement
    /// e.g. return a;
    /// return_value : a
    /// raw_content : "return a;"
    pub return_value: Option<ValueOperationToken>,
    pub raw_content: String,
}