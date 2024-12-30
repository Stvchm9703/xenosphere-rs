use super::value_operation_token;
use serde::{de, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableDeclartionToken {
    /// VariableDeclartionToken
    /// for the simple variable declaration
    /// e.g. public const int a = 10;
    /// name : a
    /// value : 10
    /// declartion_type : int
    /// prefix : public, const
    /// raw_content : "public const int a = 10;"
    pub declartion_type: String,
    pub raw_declartion_type: String,
    pub prefix: Vec<String>,
    pub raw_content: String,
    pub assignment: Option<value_operation_token::ValueOperationToken>,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclartionTypeEnum {
    String, 
    Int,
    Float,
    Bool,
    Char,
    Array,
    Object,
    Function,
    // Pointer(DeclartionTypeEnum),
    // Reference(DeclartionTypeEnum),
}