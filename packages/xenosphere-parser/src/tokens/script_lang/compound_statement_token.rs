use crate::tokens::script_lang::StatementToken;
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompoundStatementToken {
    /// CompoundStatementToken
    /// for the simple compound statement
    /// e.g. {
    ///     a = b + c;
    ///     b = a + c;
    /// }
    /// statements : [a = b + c, b = a + c]
    /// raw_content : "{a = b + c; b = a + c;}"
    pub statements: Vec<StatementToken>,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl Default for CompoundStatementToken {
    fn default() -> Self {
        CompoundStatementToken {
            statements: Vec::new(),
            raw_token: None,
        }
    }
}

impl StatementTokenTrait for CompoundStatementToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
