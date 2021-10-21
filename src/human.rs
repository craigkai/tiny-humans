use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use rusqlite::NO_PARAMS;

#[path = "database.rs"]
pub mod database;

#[derive(Debug, Deserialize, Serialize)]
pub struct Human {
  pub id: i64,
  pub x: i32,
  pub y: i32,
  pub pose: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Humans {
  pub humans: Vec<Human>
}

#[get("/")]
pub async fn get() -> Value {
  let db_connection = database::db();
  let mut statement = db_connection.prepare("select id, x, y, pose from humans;").unwrap();

  let humans_iter = statement.query_map(rusqlite::NO_PARAMS, |row| {
    Ok(Human {
        id: row.get(0)?,
        x: row.get(1)?,
        y: row.get(2)?,
        pose: row.get(3)?,
    })
  }).unwrap();

  let mut humans = Humans{ humans: Vec::new() };
  for human in humans_iter {
    humans.humans.push(human.unwrap());
  }
  json!(humans)
}

#[allow(dead_code)]
#[post("/", format = "application/json", data = "<human>")]
pub async fn new(human: Json<Human>) -> Value {
  // Adds record into db and returns current db
  // TODO: Don't grab all rows, just the newest
  let db_connection = database::db();
  db_connection
    .execute(
      "INSERT INTO humans (x, y, pose) VALUES (?1, ?2, ?3);",
      &[&human.x, &human.y, &human.pose]
  ).unwrap();

  json!(get().await)
}

#[allow(dead_code)]
#[delete("/")]
pub async fn clear() {
  let db_connection = database::db();
  db_connection
    .execute(
      "DELETE FROM humans;",
      NO_PARAMS
  ).unwrap();
}

#[allow(dead_code)]
pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
    rocket.mount("/humans", routes![get, new, clear])
  })
}
