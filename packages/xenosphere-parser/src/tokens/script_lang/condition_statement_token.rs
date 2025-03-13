use crate::tokens::script_lang::compound_statement_token::CompoundStatementToken;
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub statement_type: StatementType,
    pub set: Vec<ConditionStatementSet>,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for ConditionStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementType {
    Unknown(String),
    If,
    Switch,
}

impl Default for StatementType {
    fn default() -> Self {
        StatementType::Unknown(String::new())
    }
}

//  ---
//
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionStatementSet {
    /// ConditionStatementSet
    ///     the subset of ConditionStatementToken
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
    /// compound : CompoundStatementToken { statements: [a = b + c, b = a + c], raw_content: "a = b + c; b = a + c;" }
    /// raw_content : "{a = b + c; b = a + c;}"
    pub order: usize,
    pub statement_set_type: StatementSetType, // if, else if, else, case
    pub condition: String,
    pub compound: CompoundStatementToken,
    pub raw_content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementSetType {
    Unknown(String),
    If,
    Else,
    ElseIf,
    Case,
    Default,
}

impl Default for StatementSetType {
    fn default() -> Self {
        StatementSetType::Unknown(String::new())
    }
}
