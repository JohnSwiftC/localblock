use p256::ecdsa::{SigningKey};
use rand_core::OsRng;
use sqlite::Connection;
use sqlite::State;

/// Creates and stores a signing key
pub fn create_private_key(conn: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
    let key = SigningKey::random(&mut OsRng);
    let mut statement = conn.prepare("INSERT INTO keys (name, key) VALUES (?, ?)").unwrap();
    statement.bind((1, name)).map_err(|e| { LoadingError::GenericSQLError { message: format!("{}", e) }})?;
    statement.bind((2, key.to_bytes().as_slice())).map_err(|e| LoadingError::GenericSQLError { message: format!("{}", e) })?;
    statement.next().map_err(|e| { LoadingError::GenericSQLError { message: format!("{}", e) }})?;
    Ok(key)
}

#[derive(Debug)]
pub enum LoadingError {
    NameNotFound,
    KeyFailedLoad,
    GenericSQLError { message: String },
}

pub fn load_signing_key(connection: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
    let mut statement = connection.prepare("SELECT key FROM keys WHERE name = ?").map_err(|e| {
        LoadingError::GenericSQLError { message: format!("{}", e) }
    })?;
    statement.bind((1, name)).map_err(|e| {
        LoadingError::GenericSQLError { message: format!("{}", e) }
    })?;
    
    if let Ok(State::Row) = statement.next() {
        let key_blob: Vec<u8> = statement.read(0).map_err(|e| {
            LoadingError::GenericSQLError { message: format!("{}", e) }
        })?;
        let key = SigningKey::try_from(&key_blob[..]).map_err(|_| LoadingError::KeyFailedLoad)?;
        Ok(key)
    } else {
        Err(LoadingError::NameNotFound)
    }
}

pub fn init_db_conn() -> Result<Connection, LoadingError> {
    let conn = sqlite::open("client.db").map_err(|e| {
        LoadingError::GenericSQLError { message: format!("{}", e) }
    })?;
    conn.execute("CREATE TABLE IF NOT EXISTS keys (name TEXT PRIMARY KEY, key BLOB)").map_err(|e| {
        LoadingError::GenericSQLError { message: format!("{}", e) }
    })?;
    Ok(conn)
}