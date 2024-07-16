use crate::tokens::c_lang::{
    compound_statement_token::CompoundStatementToken,
    variable_declartion_token::VariableDeclartionToken,
};

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
    pub function_args: Vec<VariableDeclartionToken>,
    pub function_return_type: Vec<VariableDeclartionToken>,
    pub function_body: CompoundStatementToken,
    pub raw_content: String,
}
