use p256::ecdsa::{SigningKey, VerifyingKey};
use p256::pkcs8::{EncodePublicKey, EncodePrivateKey};
use rand_core::OsRng;
use sqlite::Connection;
use sqlite::State;
use std::error::Error;

/// Creates and stores a signing key
pub fn create_signing_key(conn: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
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

pub fn load_signing_key(conn: &Connection, name: &str) -> Result<SigningKey, LoadingError> {
    let mut statement = conn
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

pub fn get_signing_key_pem(conn: &Connection, name: &str) -> Result<String, LoadingError> {
    let signing_key = load_signing_key(conn, name)?;

    // I'm pretty sure unwrapping here is completely fine. I think it returns a result in the case
    // that someone implementing EncodePrivateKey writes the required function incorrectly
    // So i'm just hoping that the library properly handles in own native struct :)
    let pem = signing_key.to_pkcs8_pem(p256::pkcs8::LineEnding::CRLF).unwrap();

    Ok(pem.to_string())

}

pub fn load_verifying_key(conn: &Connection, name: &str) -> Result<VerifyingKey, LoadingError> {
    let private_key = load_signing_key(conn, name)?;
    let public_key = private_key.verifying_key().to_owned();
    Ok(public_key)
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

pub fn init_db_conn(path: &str) -> Result<Connection, LoadingError> {
    let conn = sqlite::open(path).map_err(|e| LoadingError::GenericSQLError {
        message: format!("{}", e),
    })?;
    conn.execute("CREATE TABLE IF NOT EXISTS keys (name TEXT PRIMARY KEY, key BLOB)")
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
impl Error for LoadingError {}

#[derive(Debug)]
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
impl Error for DeletionError {}

#[cfg(test)]
mod tests {

    use super::*;
    use std::error::Error;

    /// This test relies on you having a tests.db with a properly created key 'test' within
    #[test]
    fn does_key_load() -> Result<(), Box<dyn Error + 'static>> {
        let conn = init_db_conn("tests.db")?;

        let key     = load_signing_key(&conn, "test")?;

        Ok(())
    }
}