// error.rs

pub enum Error {
    AstQuery(tree_sitter::QueryError),
    AstLanguage(tree_sitter::LanguageError),
    Io(std::io::Error),
    Generic(String),
    SerdeSerializer(String),
    SerdeDeserializer(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AstQuery(e) => write!(f, "AST Query error: {}", e),
            Error::AstLanguage(e) => write!(f, "AST Language error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Generic(e) => write!(f, "Generic error: {}", e),
            Error::SerdeSerializer(e) => write!(f, "Serde Serializer error: {}", e),
            Error::SerdeDeserializer(e) => write!(f, "Serde Deserializer error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<tree_sitter::QueryError> for Error {
    fn from(e: tree_sitter::QueryError) -> Self {
        Error::AstQuery(e)
    }
}

impl From<tree_sitter::LanguageError> for Error {
    fn from(e: tree_sitter::LanguageError) -> Self {
        Error::AstLanguage(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
