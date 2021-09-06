use std::time::{SystemTime};
use std::convert::TryFrom;
use image::{GenericImageView};
use crate::responders::ImageResponse;
use crate::parser::ImageTransformation;
use crate::{MAX_SOURCE_RESOLUTION, MAX_OUTPUT_FILE_SIZE};

fn mime_type_from_image_format(format: image::ImageFormat) -> Option<String> {
  match format {
    image::ImageFormat::Jpeg => Some(String::from("image/jpeg")),
    image::ImageFormat::Png => Some(String::from("image/png")),
    _ => None,
  }
}

pub fn image_transformation(source_bytes: &[u8], transformations: String) -> Option<ImageResponse> {
  let before_transform_time = SystemTime::now();
  let source_format = image::guess_format(&source_bytes).ok()?;

  // Decode remote image into DynamicImage.
  let decoded_image = image::load_from_memory_with_format(&source_bytes, source_format).ok()?;

  // Drop files larger than x pixels.
  if (decoded_image.width() * decoded_image.height()) > MAX_SOURCE_RESOLUTION {
    println!("Image too large!");
    return None;
  }

  // Apply the requested transformations.
  let transformation = ImageTransformation::try_from(transformations.to_string()).ok()?;
  let transformed_image = transformation.apply_to(&decoded_image)?;

  // Encode the DynamicImage into bytes.
  let mut output_bytes = Vec::with_capacity(MAX_OUTPUT_FILE_SIZE);
  transformed_image.write_to(&mut output_bytes, source_format).ok()?;

  let task_duration = SystemTime::now().duration_since(before_transform_time).ok()?.as_millis();
  println!("[TRANS] ({}ms):\r\n{}", task_duration, transformations.to_string());

  Some(
    ImageResponse::new(
      mime_type_from_image_format(source_format)?.to_owned(),
      output_bytes
    )
  )
}
