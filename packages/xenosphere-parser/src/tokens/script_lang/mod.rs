// use crate::parsers::clang::Rule;
// use pest::iterators::{Pair, Pairs};
use anyhow::{Error, Result};

pub mod variable_declartion_token;
use serde::{Deserialize, Serialize};
use variable_declartion_token::VariableDeclartionToken;

pub mod value_operation_token;
use value_operation_token::ValueOperationToken;

pub mod compound_statement_token;
use compound_statement_token::CompoundStatementToken;

pub mod loop_statement_token;
use loop_statement_token::LoopStatementToken;

pub mod function_declaration_statement_token;
use function_declaration_statement_token::FunctionDeclarationStatementToken;

pub mod function_call_statement_token;
use function_call_statement_token::FunctionCallStatementToken;

pub mod return_statement_token;
use return_statement_token::ReturnStatementToken;

pub mod condition_statement_token;
use condition_statement_token::ConditionStatementToken;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScriptBlock {
    pub raw_content: String,
    pub script_syntex: String,
    pub target_platform: String,
    pub script_filename: String,
    pub parsed_tokens: Vec<StatementToken>,
    pub parser_engine: String,
}

pub trait StatementTokenParser {
    // fn try_parse(pair: Pair<Rule>) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Unknown,
}

pub trait StatementTokenTrait {
    fn get_raw_content() -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentToken {
    /// CCommentToken
    /// for the simple comment statement
    /// e.g. // this is a comment
    /// comment : this is a comment
    /// raw_content : "// this is a comment"
    pub comment: String,
    pub raw_content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDeclarationToken {
    pub raw_content: String,
    pub import_path: String,
    pub import_name: String,
}
