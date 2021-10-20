#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

pub mod database;
pub mod human;
pub mod frontend;

#[rocket::main]
async fn main() {
  {
    let db_connection = database::db();

    db_connection
        .execute(
            "create table if not exists humans (
                id integer primary key,
                x i32 not null,
                y i32 not null
            );",
            rusqlite::NO_PARAMS,
        )
        .unwrap();
  }

  if let Err(e) = rocket::build()
      .attach(human::stage())
      .attach(Template::fairing())
      .mount("/", routes![frontend::index])
      .mount("/", FileServer::from(relative!("static")))
      .launch()
      .await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
          drop(e);
      }
}
