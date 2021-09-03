#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

#[get("/remote/fetch/<remote_url>/<transformations>")]
async fn remote_fetch(remote_url: String, transformations: String) -> String {
  sleep(Duration::from_secs(1)).await;
  format!("Remote URL: {}\r\nTransformations: {}", remote_url, transformations)
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![remote_fetch])
}