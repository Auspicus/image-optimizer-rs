use std::collections::HashMap;
use std::convert::TryFrom;
use image::{DynamicImage};
use image::imageops::FilterType;

enum OperationType {
  SCALE,
  CROP,
}

fn op_type_from_string(s: &str) -> Option<OperationType> {
  match s {
    "scale" => Some(OperationType::SCALE),
    "crop" => Some(OperationType::CROP),
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
        Ok(op) => op,
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
  pub fn apply_to(&self, dynamic_image: &DynamicImage) -> Option<DynamicImage> {
    let mut result: DynamicImage = dynamic_image.to_owned();

    for op in &self.operations {
      match op.op_type {
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
      }
    }

    Some(result)
  }
}