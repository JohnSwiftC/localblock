use p256::ecdsa::SigningKey;
use rand_core::OsRng;
use sqlite::Connection;
use sqlite::State;

/// Creates and stores a signing key
pub fn create_private_key(conn: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
    let key = SigningKey::random(&mut OsRng);
    let mut statement = conn
        .prepare("INSERT INTO keys (name, key) VALUES (?, ?)")
        .unwrap();
    statement
        .bind((1, name))
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;
    statement
        .bind((2, key.to_bytes().as_slice()))
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;
    statement
        .next()
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;
    Ok(key)
}

/// Deletes a signing key with extra verifications
pub fn delete_signing_key(conn: &Connection, name: &str) -> Result<(), DeletionError> {
    println!(
        "Re-input the keys name to confirm that you wish to delete this key. THIS CANNOT BE UNDONE: "
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if name != input.trim() {
        return Err(DeletionError::NameNotConfirmed {
            name: name.to_owned(),
        });
    }

    let mut statement = conn.prepare("DELETE FROM keys WHERE name = ?").unwrap();
    statement
        .bind((1, name))
        .map_err(|e| DeletionError::GenericSQLError {
            message: format!("{}", e),
        })?;

    statement
        .next()
        .map_err(|e| DeletionError::GenericSQLError {
            message: format!("{}", e),
        })?;

    Ok(())
}

pub fn load_signing_key(connection: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
    let mut statement = connection
        .prepare("SELECT key FROM keys WHERE name = ?")
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;
    statement
        .bind((1, name))
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;

    if let Ok(State::Row) = statement.next() {
        let key_blob: Vec<u8> = statement
            .read(0)
            .map_err(|e| LoadingError::GenericSQLError {
                message: format!("{}", e),
            })?;
        let key = SigningKey::try_from(&key_blob[..]).map_err(|_| LoadingError::KeyFailedLoad)?;
        Ok(key)
    } else {
        Err(LoadingError::NameNotFound)
    }
}

pub fn get_wallet_names(conn: &Connection) -> Result<Vec<String>, LoadingError> {
    let mut statement =
        conn.prepare("SELECT name FROM keys")
            .map_err(|e| LoadingError::GenericSQLError {
                message: format!("{}", e),
            })?;

    let mut names: Vec<String> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let name: String = statement
            .read(0)
            .map_err(|e| LoadingError::GenericSQLError {
                message: format!("{}", e),
            })?;

        names.push(name);
    }

    Ok(names)
}

pub fn init_db_conn() -> Result<Connection, LoadingError> {
    let conn = sqlite::open("client.db").map_err(|e| LoadingError::GenericSQLError {
        message: format!("{}", e),
    })?;
    conn.execute("CREATE TABLE IF NOT EXISTS keys (name TEXT PRIMARY KEY, key BLOB)")
        .map_err(|e| LoadingError::GenericSQLError {
            message: format!("{}", e),
        })?;
    Ok(conn)
}

pub enum LoadingError {
    NameNotFound,
    KeyFailedLoad,
    GenericSQLError { message: String },
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
        }
    }
}

pub enum DeletionError {
    NameNotConfirmed { name: String },
    GenericSQLError { message: String },
}

impl std::fmt::Display for DeletionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeletionError::NameNotConfirmed { name } => write!(f, "failed to confirm '{}'", name),
            DeletionError::GenericSQLError { message } => write!(f, "{}", message),
        }
    }
}
