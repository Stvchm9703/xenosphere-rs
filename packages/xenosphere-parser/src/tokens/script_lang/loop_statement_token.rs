use serde::{Serialize, Deserialize};
use crate::tokens::script_lang::{
    compound_statement_token::CompoundStatementToken,
    variable_declartion_token::VariableDeclartionToken,
    condition_statement_token::ConditionStatementSet,
    value_operation_token::ValueOperationSet,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub raw_loop_condition: String,
    pub compound: CompoundStatementToken,
}