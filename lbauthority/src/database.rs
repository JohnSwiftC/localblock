use sqlite::Connection;
use std::error::Error;

pub fn init_db_conn(path: &str) -> Result<Connection, LoadingError> {
    let conn = sqlite::open(path).map_err(|e| LoadingError::GenericSQLError {
        message: format!("{}", e),
    })?;
    conn.execute("CREATE TABLE IF NOT EXISTS coins (block BLOB)")
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;

    Ok(conn)
}

#[derive(Debug)]
pub enum LoadingError {
    NameNotFound,
    KeyFailedLoad,
    GenericSQLError { message: String },
    GenericCryptoError { message: String },
    GenericFileError { message: String },
}

impl std::fmt::Display for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingError::NameNotFound => write!(f, "key is not present in the database"),
            LoadingError::KeyFailedLoad => write!(
                f,
                "key is not properly formatted in the database, erase the entry and attempt to restore"
            ),
            LoadingError::GenericSQLError { message } => write!(f, "{}", message),
            LoadingError::GenericCryptoError { message } => write!(f, "{}", message),
            LoadingError::GenericFileError { message } => write!(f, "{}", message),
        }
    }
}
impl Error for LoadingError {}
