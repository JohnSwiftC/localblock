use sqlite::Connection;
fn main() {
    let connection = load_db();
}

fn load_db() -> Connection {
    sqlite::open("client.db").expect("Unable to open local database...")
}
