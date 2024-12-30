use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub assigned: String,
    pub raw_content: String,
    pub value_operation_set: Vec<ValueOperationSet>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueOperationSet {
    /// ValueOperationSet
    /// for the simple value operation set
    /// e.g. a = b + c + d + e();
    /// assigned : a
    /// value : b
    /// operator : =
    /// raw_content : "= b"
    pub order: i32,
    pub operate_type: String, // = + - * / % & | ^ << >> && || < <= > >= == !=
    pub value: String,
    pub raw_content: String,
}
