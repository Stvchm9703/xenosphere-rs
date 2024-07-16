use crate::parsers::clang::Rule;
use anyhow::{Error, Result};
use pest::iterators::{Pair, Pairs};

mod variable_declartion_token;
use variable_declartion_token::VariableDeclartionToken;

mod value_operation_token;
use value_operation_token::ValueOperationToken;

mod compound_statement_token;
use compound_statement_token::CompoundStatementToken;

mod loop_statement_token;
use loop_statement_token::LoopStatementToken;

mod function_declaration_statement_token;
use function_declaration_statement_token::FunctionDeclarationStatementToken;

mod function_call_statement_token;
use function_call_statement_token::FunctionCallStatementToken;

mod function_return_statement_token;
use function_return_statement_token::FunctionReturnStatementToken;

mod condition_statement_token;
use condition_statement_token::ConditionStatementToken;

pub struct CLangScriptBlock {
    pub raw_content: String,
    pub script_syntex: String,
    pub target_platform: String,
    pub script_filename: String,
    pub parsed_tokens: Vec<StatementToken>,
}

pub trait StatementTokenParser {
    fn try_parse(pair: Pair<Rule>) -> Result<(), Error>;
}

pub enum StatementToken {
    VariableDeclaration(VariableDeclartionToken),
    ValueOperationStatment(ValueOperationToken),
    CompoundStatement(CompoundStatementToken),
    LoopStatement(LoopStatementToken),
    FunctionDeclarationStatement(FunctionDeclarationStatementToken),
    FunctionCallStatement(FunctionCallStatementToken),
    FunctionReturnStatement(FunctionReturnStatementToken),
    ConditionStatement(ConditionStatementToken),
    Comment(CommentToken),
}

pub struct CommentToken {
    /// CCommentToken
    /// for the simple comment statement
    /// e.g. // this is a comment
    /// comment : this is a comment
    /// raw_content : "// this is a comment"
    pub comment: String,
    pub raw_content: String,
}
