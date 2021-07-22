mod csv_handler;
mod database_maneger;
use rusqlite::Connection;

fn main() {
    let conn = Connection::open("6k Korean words.csv").unwrap();
}
