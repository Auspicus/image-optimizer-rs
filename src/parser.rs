use std::collections::HashMap;
use std::convert::TryFrom;
use image::{DynamicImage};
use image::imageops::FilterType;
use crate::{MAX_OUTPUT_FILE_SIZE};
use crate::responders::ImageResponse;

enum OperationType {
  SCALE,
  CROP,
  FORMAT,
  NOOP,
}

fn mime_type_from_image_format(f: image::ImageFormat) -> Option<String> {
  match f {
    image::ImageFormat::Jpeg => Some(String::from("image/jpeg")),
    image::ImageFormat::Png => Some(String::from("image/png")),
    _ => None,
  }
}

fn image_format_from_string(s: &str) -> Option<image::ImageFormat> {
  match s {
    "jpeg" => Some(image::ImageFormat::Jpeg),
    "png" => Some(image::ImageFormat::Png),
    _ => None,
  }
}

fn op_type_from_string(s: &str) -> Option<OperationType> {
  match s {
    "scale" => Some(OperationType::SCALE),
    "crop" => Some(OperationType::CROP),
    "fmt" => Some(OperationType::FORMAT),
    "noop" => Some(OperationType::NOOP),
    _ => None,
  }
}

struct Operation<T> {
  op_type: T,
  parameters: HashMap<String, String>,
}

impl TryFrom<Vec<&str>> for Operation<OperationType> {
  type Error = &'static str;

  fn try_from(strings: Vec<&str>) -> Result<Self, Self::Error> {
    let mut parameters: HashMap<String, String> = HashMap::new();

    for p in strings {
      let parts = p.split("_").collect::<Vec<_>>();
      if parts.len() != 2 {
        return Err("Invalid operation")
      }
      
      parameters.insert(parts[0].to_string(), parts[1].to_string());
    }

    let op_str = match parameters.get("op") {
      Some(v) => v,
      None => {
        return Err("Operation type not provided")
      }
    };

    let op_type = match op_type_from_string(op_str) {
      Some(v) => v,
      None => {
        return Err("Unknown operation type")
      }
    };

    Ok(Operation {
      op_type,
      parameters,
    })
  }
}

pub struct ImageTransformation {
  operations: Vec<Operation<OperationType>>
}

impl TryFrom<String> for ImageTransformation {
  type Error = &'static str;

  fn try_from(string: String) -> Result<Self, Self::Error> {
    let mut operations: Vec<Operation<OperationType>> = Vec::new();

    for op_s in string.split("|") {
      let op = match Operation::try_from(op_s.split(",").collect::<Vec<_>>()) {
        Ok(o) => o,
        Err(_) => {
          return Err("Invalid operation")
        }
      };
      
      operations.push(op);
    }

    Ok(ImageTransformation { operations })
  }
}

impl ImageTransformation {
  pub fn apply_to(&self, source_format: &image::ImageFormat, dynamic_image: &DynamicImage) -> Option<ImageResponse> {
    let mut output_format = source_format.to_owned();
    let mut result: DynamicImage = dynamic_image.to_owned();

    for op in &self.operations {
      match op.op_type {
        OperationType::NOOP => {},
        OperationType::SCALE => {
          let w = op.parameters.get("w")?.parse::<u32>().ok()?;
          let h = op.parameters.get("h")?.parse::<u32>().ok()?;

          result = result.resize(w, h, FilterType::Triangle);
        },
        OperationType::CROP => {
          let x = op.parameters.get("x")?.parse::<u32>().ok()?;
          let y = op.parameters.get("y")?.parse::<u32>().ok()?;
          let w = op.parameters.get("w")?.parse::<u32>().ok()?;
          let h = op.parameters.get("h")?.parse::<u32>().ok()?;

          result = result.crop_imm(x, y, w, h);
        },
        OperationType::FORMAT => {
          let f = op.parameters.get("f")?;

          output_format = image_format_from_string(f)?;
        },
      }
    }

    // Encode the DynamicImage into bytes.
    let mut output_bytes = Vec::with_capacity(MAX_OUTPUT_FILE_SIZE);
    let output_mime_type = mime_type_from_image_format(output_format)?;

    match output_format {
      _ => {
        result.write_to(&mut output_bytes, output_format).ok()?;
      }
    }

    Some(ImageResponse::new(output_mime_type, output_bytes))
  }
}