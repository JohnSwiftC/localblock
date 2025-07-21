mod database;
fn main() {
    let connection = database::init_db_conn().unwrap();
    database::create_private_key(&connection, "testkey").unwrap();
    let key = database::load_signing_key(&connection, "testkey").unwrap();
}