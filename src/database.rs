use rusqlite::Connection;

#[allow(unused_imports)]
use rusqlite::ToSql;

pub fn db() -> rusqlite::Connection {
  let db_connection = Connection::open("data.sqlite").unwrap();
  let _journal_mode = db_connection.pragma_update(None, "journal_mode", &"WAL");
  db_connection
}

#[allow(dead_code)]
pub fn create() {
  let db_connection = db();

  db_connection
    .execute(
        "create table if not exists humans (
            id integer primary key,
            x i32 not null,
            y i32 not null,
            pose i32 not null,
            color TEXT not null
        );",
        [],
    ).unwrap();
}

#[test]
fn grab_new_row() {
    let conn = Connection::open_in_memory().expect("Could not test: DB not created");

    conn.execute("create table if not exists humans (
      id integer primary key,
      x i32 not null,
      y i32 not null,
      pose i32 not null,
      color TEXT not null
  );", []).expect("Creation failure");

    let color = "blue".to_string();
    conn.execute(
      "INSERT INTO humans (color, x, y, pose) VALUES (?1, ?2, ?3, ?4)",
      &[&color as &dyn ToSql, &0, &0, &1]).unwrap();

    let last_id = conn.last_insert_rowid();
    assert_eq!(last_id, 1);
}
