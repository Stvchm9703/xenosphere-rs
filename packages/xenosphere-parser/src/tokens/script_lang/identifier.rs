use crate::tokens::script_lang::{
    compound_statement_token::CompoundStatementToken,
    variable_declartion_token::VariableDeclartionToken,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierToken {
    /// CIdentifierToken
    /// for the simple identifier token
    /// e.g. a
    /// identifier : a
    /// raw_content : "a"
    pub identifier: String,
    pub raw_content: String,
}