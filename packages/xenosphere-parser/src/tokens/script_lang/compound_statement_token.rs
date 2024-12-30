
use crate::tokens::script_lang::StatementToken;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompoundStatementToken {
    /// CCompoundStatementToken
    /// for the simple compound statement
    /// e.g. {
    ///     a = b + c;
    ///     b = a + c;
    /// }
    /// statements : [a = b + c, b = a + c]
    /// raw_content : "{a = b + c; b = a + c;}"
    pub statements: Vec<StatementToken>,
    pub raw_content: String,
}