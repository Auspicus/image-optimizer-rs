use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};

pub struct ImageResponse {
  mime_type: String,
  bytes: Vec<u8>,
}

impl ImageResponse {
  pub fn new(mime_type: String, bytes: Vec<u8>) -> Self {
    Self {
      mime_type,
      bytes,
    }
  }
}

// @todo: Cache-Control header on 404 to prevent continuous failover.
impl<'r> Responder<'r, 'static> for ImageResponse {
  fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
    Response::build()
      .sized_body(self.bytes.len(), Cursor::new(self.bytes))
      .raw_header("Content-Type", self.mime_type)
      .raw_header("Cache-Control", "public, max-age=86400")
      .ok()
  }
}
