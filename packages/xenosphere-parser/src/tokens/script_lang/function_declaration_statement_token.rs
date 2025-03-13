use crate::tokens::script_lang::{
    compound_statement_token::CompoundStatementToken,
    variable_declartion_token::VariableDeclartionToken,
};
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclarationStatementToken {
    /// CFunctionDeclarationStatementToken
    /// for the simple function declaration statement
    /// e.g. int add(int a, int b) {
    ///     return a + b;
    /// }
    /// function_name : add
    /// function_args : [int a, int b]
    /// function_return_type : int
    /// function_body : [return a + b]
    /// raw_content : "int add(int a, int b) {return a + b;}"
    pub function_name: String,
    pub function_namespace: String,
    pub function_args: Vec<VariableDeclartionToken>,
    pub function_return_type: Vec<VariableDeclartionToken>,
    pub function_body: CompoundStatementToken,
    // pub raw_content: String,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for FunctionDeclarationStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
