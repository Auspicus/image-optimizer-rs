#[macro_use] extern crate rocket;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5 MiB.

use std::io::Cursor;
use std::time::{SystemTime};
use std::mem;
use std::cmp::Ordering;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use image::imageops::FilterType;

#[derive(Clone)]
struct ImageResponse {
  mime_type: String,
  bytes: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for ImageResponse {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
      .sized_body(self.bytes.len(), Cursor::new(self.bytes))
      .raw_header("Content-Type", self.mime_type)
      .ok()
  }
}

#[get("/")]
async fn index() -> String {
  String::from("Health? Check!")
}

fn mime_type_from_image_format(format: image::ImageFormat) -> Option<String> {
  match format {
    image::ImageFormat::Jpeg => Some(String::from("image/jpeg")),
    image::ImageFormat::Png => Some(String::from("image/png")),
    _ => None,
  }
}

#[get("/remote/fetch/<remote_url>/<transformations>")]
async fn remote_fetch(remote_url: String, transformations: String) -> Option<ImageResponse> {
  let before_request_time = SystemTime::now();
  let response = match reqwest::get(remote_url).await {
    Ok(v) => v,
    Err(_) => {
      println!("Failed to get image");
      return None
    },
  };
  let after_image_fetch = SystemTime::now();
  let task_duration = after_image_fetch.duration_since(before_request_time).ok()?.as_millis();
  println!("Fetch: {}ms", task_duration);

  // Create a byte buffer of the raw image data.
  let body = match response.bytes().await {
    Ok(v) => v,
    Err(_) => {
      println!("Failed to parse image bytes");
      return None
    },
  };

  // Inspect remote image.
  let source_bytes = body.as_ref();

  // Drop files larger than x.
  if source_bytes.len().cmp(&MAX_FILE_SIZE) == Ordering::Greater {
    return None;
  }

  let source_format = image::guess_format(&source_bytes).ok()?;

  // Decode remote image into DynamicImage.
  let decoded_image = image::load_from_memory_with_format(&source_bytes, source_format).ok()?;

  // Transform the image arbitrarily.
  let transformed_image = decoded_image.resize_to_fill(
    300,
    300,
    FilterType::Triangle
  );

  mem::drop(decoded_image);

  // Encode the DynamicImage into bytes.
  let mut output_bytes = Vec::with_capacity(transformed_image.as_bytes().len());
  transformed_image.write_to(&mut output_bytes, source_format).ok();

  mem::drop(transformed_image);

  let task_duration = SystemTime::now().duration_since(after_image_fetch).ok()?.as_millis();
  println!("Transform: {}ms", task_duration);

  let mime_type = match mime_type_from_image_format(source_format) {
    Some(v) => v,
    None => return None,
  };

  Some(ImageResponse {
    mime_type: mime_type.to_owned(),
    bytes: output_bytes,
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![
      index,
      remote_fetch,
    ])
}