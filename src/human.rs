use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use rusqlite::NO_PARAMS;
use rusqlite::types::ToSql;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
use rocket::{State, Shutdown};
use rocket::tokio::sync::broadcast::{Sender, error::RecvError};

#[path = "database.rs"]
pub mod database;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub update: bool
}

// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[allow(dead_code)]
#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Human {
  pub id: i64,
  pub x: i32,
  pub y: i32,
  pub pose: i32,
  pub color: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Humans {
  pub humans: Vec<Human>
}

#[get("/")]
pub async fn get() -> Value {
  let db_connection = database::db();
  let mut statement = db_connection.prepare("select id, x, y, pose, color from humans;").unwrap();

  let humans_iter = statement.query_map(rusqlite::NO_PARAMS, |row| {
    Ok(Human {
        id: row.get(0)?,
        x: row.get(1)?,
        y: row.get(2)?,
        pose: row.get(3)?,
        color: row.get(4)?
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
pub async fn new(human: Json<Human>, queue: &State<Sender<Message>>) -> Value {
  // Adds record into db and returns the new row

  let db_connection = database::db();
  db_connection
    .execute(
      "INSERT INTO humans (color, x, y, pose) VALUES (?1, ?2, ?3, ?4);",
      &[&human.color as &dyn ToSql, &human.x, &human.y, &human.pose]
  ).unwrap();

  let id = db_connection.last_insert_rowid();

  let mut stmt = db_connection.prepare("SELECT id, x, y, pose, color FROM humans WHERE id=:id;").unwrap();
  let human_iter = stmt.query_map_named(&[(":id", &id.to_string())], |row| {
    Ok(Human {
      id: row.get(0)?,
      x: row.get(1)?,
      y: row.get(2)?,
      pose: row.get(3)?,
      color: row.get(4)?
    })
  }).unwrap();

  let mut res = Vec::new();
  for human in human_iter {
    res.push( human.unwrap() );
    break;
  }
  let _res = queue.send(Message{update: true});
  json!(res.pop())
}

#[allow(dead_code)]
#[delete("/")]
pub async fn clear(queue: &State<Sender<Message>>) {
  let db_connection = database::db();
  db_connection
    .execute(
      "DELETE FROM humans;",
      NO_PARAMS
  ).unwrap();
  let _res = queue.send(Message{update: true});
}

#[allow(dead_code)]
pub fn stage() -> rocket::fairing::AdHoc {
  rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
    rocket.mount("/humans", routes![get, new, clear])
  })
}
