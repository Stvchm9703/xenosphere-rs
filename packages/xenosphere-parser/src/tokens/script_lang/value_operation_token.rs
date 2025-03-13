use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueOperationToken {
    /// ValueOperationToken
    /// for the simple value operation
    /// e.g. a = b + c;
    /// assigned : a
    /// value : b + c
    /// operator : +
    /// raw_content : "a = b + c;"
    // debug: String,
    // debug_operate_type: String,
    pub assigned_variable: String,
    // pub raw_content: String,
    pub value_operation_set: Vec<ValueOperationSet>,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for ValueOperationToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueOperationSet {
    /// ValueOperationSet
    /// for the simple value operation set
    /// e.g. a = b + c + d + e();
    /// assigned : a
    /// value : b
    /// operator : =
    /// raw_content : "= b"
    pub order: usize,
    pub operate_type: String, // = + - * / % & | ^ << >> && || < <= > >= == !=
    pub value: String,
    pub raw_content: String,
}
