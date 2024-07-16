use crate::tokens::c_lang::variable_declartion_token::VariableDeclartionToken;

pub struct FunctionCallStatementToken {
    /// CFunctionCallStatementToken
    /// for the simple function call statement
    /// e.g. add(a, b);
    /// function_name : add
    /// function_args : [a, b]
    /// raw_content : "add(a, b);"
    pub function_name: String,
    pub function_args: Vec<VariableDeclartionToken>,
    pub raw_content: String,
}
