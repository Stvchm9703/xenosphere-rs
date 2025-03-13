use crate::tokens::script_lang::{
    compound_statement_token::CompoundStatementToken,
    condition_statement_token::ConditionStatementSet, value_operation_token::ValueOperationSet,
    variable_declartion_token::VariableDeclartionToken, UnalignedToken,
};
use serde::{Deserialize, Serialize};

use super::StatementTokenTrait;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopStatementToken {
    /// CLoopStatementToken
    /// for the simple loop statement
    /// e.g. for (int i = 0; i < 10; i++) {
    ///     a = b + c;
    ///     b = a + c;
    /// }
    /// loop_type : for
    /// loop_declaration : int i = 0;
    /// loop_condition : i < 10;
    /// loop_update : i++
    /// statements : [a = b + c, b = a + c]
    /// raw_content : "for (int i = 0; i < 10; i++) {a = b + c; b = a + c;}"
    pub loop_type: String,
    pub loop_declaration: VariableDeclartionToken,
    pub loop_condition: ConditionStatementSet,
    pub loop_update: ValueOperationSet,

    pub raw_loop_statement: String,
    pub compound: CompoundStatementToken,

    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for LoopStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
