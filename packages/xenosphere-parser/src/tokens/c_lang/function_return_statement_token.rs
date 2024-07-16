use crate::tokens::c_lang::value_operation_token::ValueOperationToken;
pub struct FunctionReturnStatementToken {
    /// CFunctionReturnStatementToken
    /// for the simple function return statement
    /// e.g. return a;
    /// return_value : a
    /// raw_content : "return a;"
    pub return_value: Option<ValueOperationToken>,
    pub raw_content: String,
}