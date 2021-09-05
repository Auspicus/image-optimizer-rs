#[macro_use] extern crate rocket;

// @todo: Cache-Control header on 404 to prevent continuous failover.

const MAX_SOURCE_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MiB.
const MAX_SOURCE_RESOLUTION: u32 = 25 * 1000000; // 25 Megapixels.
const MAX_OUTPUT_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 Megapixels.
// See: https://support.cloudinary.com/hc/en-us/articles/202520592-Do-you-have-a-file-size-limit-

use std::io::Cursor;
use std::time::{SystemTime};
use std::mem;
use std::cmp::Ordering;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use image::imageops::FilterType;
use image::{GenericImageView};

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
      .raw_header("Cache-Control", "public, max-age=86400")
      .ok()
  }
}

#[get("/")]
async fn health_check() -> String {
  String::from("Health? ✅")
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
  let response = reqwest::get(remote_url.to_owned()).await.ok()?;
  let after_image_fetch = SystemTime::now();
  let task_duration = after_image_fetch.duration_since(before_request_time).ok()?.as_millis();
  println!("FETCH: {} ({}ms)", remote_url, task_duration);

  // Create a byte buffer of the raw image data.
  let body = response.bytes().await.ok()?;

  // Inspect remote image.
  let source_bytes = body.as_ref();

  // Drop files larger than x bytes.
  if source_bytes.len().cmp(&MAX_SOURCE_FILE_SIZE) == Ordering::Greater {
    return None;
  }

  let source_format = image::guess_format(&source_bytes).ok()?;

  // Decode remote image into DynamicImage.
  let decoded_image = image::load_from_memory_with_format(&source_bytes, source_format).ok()?;

  // Drop files larger than x pixels.
  if (decoded_image.width() * decoded_image.height()).cmp(&MAX_SOURCE_RESOLUTION) == Ordering::Greater {
    return None;
  }

  // Transform the image arbitrarily.
  let transformed_image = decoded_image.resize_to_fill(
    300,
    300,
    FilterType::Triangle
  );

  // @todo: look into performance implications of this:
  mem::drop(decoded_image);

  // Encode the DynamicImage into bytes.
  let mut output_bytes = Vec::with_capacity(MAX_OUTPUT_FILE_SIZE);
  transformed_image.write_to(&mut output_bytes, source_format).ok()?;

  // @todo: look into performance implications of this:
  mem::drop(transformed_image);

  let task_duration = SystemTime::now().duration_since(after_image_fetch).ok()?.as_millis();
  println!("TRANS: {}ms", task_duration);

  let mime_type = mime_type_from_image_format(source_format)?;

  Some(ImageResponse {
    mime_type: mime_type.to_owned(),
    bytes: output_bytes,
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![
      health_check,
      remote_fetch,
    ])
}