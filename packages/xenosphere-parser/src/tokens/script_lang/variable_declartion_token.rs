use super::{value_operation_token::ValueOperationToken, StatementTokenTrait, UnalignedToken};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableDeclartionToken {
    /// VariableDeclartionToken
    /// for the simple variable declaration
    /// e.g. public const int a = 10;
    /// name : a
    /// value : 10
    /// declartion_type : int
    /// prefix : public, const
    /// raw_content : "public const int a = 10;"
    pub declartion_type: Vec<DeclartionTypeEnum>,
    pub raw_declartion_type: String,
    pub prefix: Vec<String>,
    pub assignment_token: Option<ValueOperationToken>,

    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}
impl StatementTokenTrait for VariableDeclartionToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeclartionTypeEnum {
    String,
    Int,
    Float,
    Bool,
    Char,
    Array,
    Object,
    Function,
    Auto,
    Unknown,
    Pointer,
    Reference,
}

impl Default for DeclartionTypeEnum {
    fn default() -> Self {
        DeclartionTypeEnum::Unknown
    }
}
