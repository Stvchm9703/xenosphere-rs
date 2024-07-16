use crate::tokens::c_lang::compound_statement_token::CompoundStatementToken;
pub struct LoopStatementToken {
    /// CLoopStatementToken
    /// for the simple loop statement
    /// e.g. for (int i = 0; i < 10; i++) {
    ///     a = b + c;
    ///     b = a + c;
    /// }
    /// loop_type : for
    /// loop_condition : int i = 0; i < 10; i++
    /// statements : [a = b + c, b = a + c]
    /// raw_content : "for (int i = 0; i < 10; i++) {a = b + c; b = a + c;}"
    pub loop_type: String,
    pub loop_condition: String,
    pub raw_loop_condition: String,
    pub compound: CompoundStatementToken,
}