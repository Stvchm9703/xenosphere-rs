use crate::tokens::script_lang::value_operation_token::ValueOperationToken;
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatementToken {
    /// CFunctionReturnStatementToken
    /// for the simple function return statement
    /// e.g. return a;
    /// return_value : a
    /// raw_content : "return a;"
    pub return_value: Option<ValueOperationToken>,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for ReturnStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
