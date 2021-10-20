use rusqlite::Connection;

pub fn db() -> rusqlite::Connection {
  Connection::open("data.sqlite").unwrap()
}
