use p256::ecdsa::{SigningKey};
use rand_core::OsRng;
use sqlite::Connection;
use sqlite::State;

/// Creates and stores a signing key
fn create_private_key(conn: &Connection, name: &str) -> SigningKey {
    let key = SigningKey::random(&mut OsRng);
    let mut statement = conn.prepare("INSERT INTO keys (name, key) VALUES (?, ?)").unwrap();
    statement.bind((1, name)).unwrap();
    statement.bind((2, key.to_bytes().as_slice())).unwrap();
    key
}

pub enum LoadingError {
    NameNotFound,
    KeyFailedLoad,
    GenericSQLError { message: String },
}

fn load_signing_key_hex(connection: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
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

fn init_db_conn() -> Connection {
    let conn = sqlite::open("client.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS keys (name TEXT, key BLOB)").unwrap();
    conn
}