// use crate::parsers::clang::Rule;
// use pest::iterators::{Pair, Pairs};
pub use serde::{Deserialize, Serialize};

pub mod identifier;
pub use identifier::*;

pub mod variable_declartion_token;
pub use variable_declartion_token::*;

pub mod value_operation_token;
pub use value_operation_token::*;

pub mod compound_statement_token;
pub use compound_statement_token::*;

pub mod loop_statement_token;
pub use loop_statement_token::*;

pub mod function_declaration_statement_token;
pub use function_declaration_statement_token::*;

pub mod function_call_statement_token;
pub use function_call_statement_token::*;

pub mod return_statement_token;
pub use return_statement_token::*;

pub mod condition_statement_token;
pub use condition_statement_token::*;

pub mod unaligned_token;
pub use unaligned_token::*;

pub mod comment_token;
pub use comment_token::*;

pub mod import_declaration_token;
pub use import_declaration_token::*;

pub mod error;
pub use error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScriptBlock {
    pub raw_content: String,
    pub script_syntex: String,
    pub target_platform: String,
    pub script_filename: String,
    pub parsed_tokens: Vec<StatementToken>,
    pub parser_engine: String,
}

pub trait StatementTokenParser {
    fn try_parse(raw_content: String) -> Result<ScriptBlock, Error>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementToken {
    VariableDeclaration(VariableDeclartionToken),
    ValueOperationStatment(ValueOperationToken),
    CompoundStatement(CompoundStatementToken),
    LoopStatement(LoopStatementToken),
    FunctionDeclarationStatement(FunctionDeclarationStatementToken),
    FunctionCallStatement(FunctionCallStatementToken),
    FunctionReturnStatement(ReturnStatementToken),
    ConditionStatement(ConditionStatementToken),
    Comment(CommentToken),
    ImportDeclaration(ImportDeclarationToken),
    Unknown(UnalignedToken),
}

pub trait StatementTokenTrait {
    fn get_raw_content(&self) -> &str;
}
