#[macro_use] extern crate rocket;

use std::io;
use rocket::tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
struct ImageFetchError;
type ImageFetchResult = std::result::Result<Vec<u8>, ImageFetchError>;

// impl ImageFetchError {

// }

#[get("/")]
async fn index() -> String {
  format!("Hello, world!")
}

async fn get_image_bytes(remote_url: &String) -> ImageFetchResult {
  let res = reqwest::get(remote_url).await?;
  
  Ok(vec![0])
}

#[get("/remote/fetch/<remote_url>/<transformations>")]
async fn remote_fetch(remote_url: String, transformations: String) -> io::Result<String> {
  let bytes = get_image_bytes(&remote_url).await?;
  
  println!("Remote URL: {}\r\nTransformations: {}", remote_url, transformations);

  Ok(res)
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index])
}