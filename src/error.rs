use std::fmt;
use tokio_postgres::Error as PgError;

#[derive(Debug)]
pub enum OrmError {
    DatabaseError(PgError),
    ConnectionError(String),
    QueryError(String),
    ParseError(String),
    IoError(std::io::Error),
    EnvError(std::env::VarError),
}

impl fmt::Display for OrmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrmError::DatabaseError(e) => write!(f, "Database error: {}", e),
            OrmError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            OrmError::QueryError(e) => write!(f, "Query error: {}", e),
            OrmError::ParseError(e) => write!(f, "Parse error: {}", e),
            OrmError::IoError(e) => write!(f, "I/O error: {}", e),
            OrmError::EnvError(e) => write!(f, "Environment variable error: {}", e),
        }
    }
}

impl std::error::Error for OrmError {}

impl From<PgError> for OrmError {
    fn from(err: PgError) -> OrmError {
        OrmError::DatabaseError(err)
    }
}

impl From<std::io::Error> for OrmError {
    fn from(err: std::io::Error) -> OrmError {
        OrmError::IoError(err)
    }
}

impl From<std::env::VarError> for OrmError {
    fn from(err: std::env::VarError) -> OrmError {
        OrmError::EnvError(err)
    }
}