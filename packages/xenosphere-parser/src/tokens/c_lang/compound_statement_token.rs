
use crate::tokens::c_lang::StatementToken;

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