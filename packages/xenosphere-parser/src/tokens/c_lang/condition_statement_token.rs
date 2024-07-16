use crate::tokens::c_lang::compound_statement_token::CompoundStatementToken;

pub struct ConditionStatementToken {
    /// CConditionStatementToken
    /// for the simple condition statement
    /// e.g. if (a > b) {
    ///     a = b + c;
    ///     b = a + c;
    /// } else {
    ///     a = b - c;
    ///     b = a - c;
    /// }
    /// condition : a > b
    /// if_statements : [a = b + c, b = a + c]
    /// else_statements : [a = b - c, b = a - c]
    /// raw_content : "if (a > b) {a = b + c; b = a + c;} else {a = b - c; b = a - c;}"
    pub statement_type: String, // if, switch
    pub set: Vec<ConditionStatementSet>,
    pub raw_content: String,
}

pub struct ConditionStatementSet {
    /// ConditionStatementSet
    ///     the subset of CConditionStatementToken
    /// for the simple condition statement
    /// e.g. if (a > b) {
    ///     a = b + c;
    ///     b = a + c;
    /// } else {
    ///     a = b - c;
    ///     b = a - c;
    /// }
    /// order: 0
    /// statement_set_type: if
    /// condition : a > b
    /// compound : CCompoundStatementToken { statements: [a = b + c, b = a + c], raw_content: "a = b + c; b = a + c;" }
    /// raw_content : "{a = b + c; b = a + c;}"
    pub order: i32,
    pub statement_set_type: String, // if, else if, else, case
    pub condition: String,
    pub raw_condition: String,
    pub compound: CompoundStatementToken,
    pub raw_content: String,
}
