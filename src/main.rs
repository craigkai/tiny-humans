#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use rocket::tokio::sync::broadcast::channel;

pub mod database;
pub mod human;
pub mod frontend;

#[rocket::main]
async fn main() {
  database::create();

  if let Err(e) = rocket::build()
      .manage(channel::<human::Message>(1024).0)
      .attach(human::stage())
      .attach(Template::fairing())
      .mount("/", routes![frontend::index, human::events])
      .mount("/", FileServer::from(relative!("static")))
      .launch()
      .await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
          drop(e);
      }
}
