use rocket_dyn_templates::Template;

#[path = "human.rs"]
mod human;

use human::get;

#[get("/")]
pub async fn index() -> Template {
  let humans = get().await;

  Template::render("index", &humans)
}
