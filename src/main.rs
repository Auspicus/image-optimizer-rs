#[macro_use] extern crate rocket;

use rocket::serde::json::{Json};
use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
  name: String
}

#[get("/")]
fn todo() -> Json<Task> {
    Json(Task { name: "Hello, world".to_string() })
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![todo])
}