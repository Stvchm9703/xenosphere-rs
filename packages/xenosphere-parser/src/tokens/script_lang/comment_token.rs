use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentToken {
    /// CommentToken
    /// for the simple comment statement
    /// e.g. // this is a comment
    /// comment : this is a comment
    /// raw_content : "// this is a comment"
    pub comment: String,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl Default for CommentToken {
    fn default() -> Self {
        CommentToken {
            comment: String::new(),
            raw_token: None,
        }
    }
}

impl StatementTokenTrait for CommentToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}

impl CommentToken {
    pub fn from_unaligned_token(token: UnalignedToken) -> Self {
        CommentToken {
            comment: token.raw.to_owned(),
            raw_token: Some(Box::new(token)),
        }
    }
}
