use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[path = "database.rs"]
pub mod database;

#[derive(Debug, Deserialize, Serialize)]
pub struct Human {
  pub id: i64,
  pub x: i32,
  pub y: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Humans {
  pub humans: Vec<Human>
}

#[get("/")]
pub async fn get() -> Value {
  let db_connection = database::db();
  let mut statement = db_connection.prepare("select id, x, y from humans;").unwrap();

  let humans_iter = statement.query_map(rusqlite::NO_PARAMS, |row| {
    Ok(Human {
        id: row.get(0)?,
        x: row.get(1)?,
        y: row.get(2)?,
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
pub async fn new(human: Json<Human>) {
  let db_connection = database::db();
  db_connection
    .execute(
      "INSERT INTO humans (x, y) VALUES (?1, ?2);",
      &[&human.x, &human.y]
  ).unwrap();
}

#[allow(dead_code)]
pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
    rocket.mount("/humans", routes![get, new])
  })
}
