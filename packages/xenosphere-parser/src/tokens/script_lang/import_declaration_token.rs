// import_declaration_token.rs
use serde::{Deserialize, Serialize};

use super::{StatementTokenTrait, UnalignedToken};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportDeclarationToken {
    /// ImportDeclarationToken
    /// for the import declaration statement
    /// e.g. import "path/to/module" as module_name;
    /// import_path : "path/to/module"
    /// import_name : "module_name"
    pub import_path: String,
    pub import_name: String,
    #[serde(skip)]
    pub raw_token: Option<Box<UnalignedToken>>,
}

impl StatementTokenTrait for ImportDeclarationToken {
    fn get_raw_content(&self) -> &str {
        if let Some(rt) = self.raw_token.as_ref() {
            &rt.raw
        } else {
            ""
        }
    }
}
