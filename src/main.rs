#[macro_use] extern crate rocket;

// See: https://support.cloudinary.com/hc/en-us/articles/202520592-Do-you-have-a-file-size-limit-
pub const MAX_SOURCE_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MiB.
pub const MAX_SOURCE_RESOLUTION: u32 = 25 * 1000000; // 25 Megapixels.
pub const MAX_OUTPUT_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 Megapixels.

mod handlers;
mod responders;
mod parser;

use std::time::{SystemTime};
use reqwest::header::{CONTENT_LENGTH};
use crate::responders::ImageResponse;
use crate::handlers::image_transformation;

#[get("/")]
async fn health_check() -> String {
  String::from("Health? ✅")
}

#[get("/remote/fetch/<remote_url>/<transformations>")]
async fn remote_fetch(remote_url: String, transformations: String) -> Option<ImageResponse> {
  let before_request_time = SystemTime::now();
  let response = reqwest::get(remote_url.to_owned()).await.ok()?;
  let after_image_fetch = SystemTime::now();
  let task_duration = after_image_fetch.duration_since(before_request_time).ok()?.as_millis();
  println!("[FETCH] ({}ms):\r\n{}", task_duration, remote_url);

  if response.headers()[CONTENT_LENGTH].to_str().ok()?.parse::<usize>().ok()? > MAX_SOURCE_FILE_SIZE {
    println!("Image too large.");
    return None;
  }

  // Create a byte buffer of the raw image data.
  let body = response.bytes().await.ok()?;

  // Inspect remote image.
  let source_bytes = body.as_ref();

  // Drop files larger than x bytes.
  if source_bytes.len() > MAX_SOURCE_FILE_SIZE {
    return None;
  }

  image_transformation(source_bytes, transformations)
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![
      health_check,
      remote_fetch,
    ])
}