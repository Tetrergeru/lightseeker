use crate::geometry::transform::RawTransform;

use super::{obj_lines, parse_transform};

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
}

pub struct AnimationFrame {
    pub transforms: Vec<RawTransform>,
}

impl AnimationFrame {
    fn new() -> Self {
        Self { transforms: vec![] }
    }
}

impl Animation {
    pub fn parse(file: &str) -> Self {
        let mut frames = vec![];
        let mut current_frame = AnimationFrame::new();
        let mut skip = true;
        for split in obj_lines(file) {
            match split[0] {
                "fr" => {
                    if skip {
                        skip = false;
                    } else {
                        frames.push(current_frame);
                        current_frame = AnimationFrame::new();
                    }
                }
                "af" => current_frame.transforms.push(parse_transform(&split[1..])),
                _ => continue,
            }
        }
        frames.push(current_frame);
        Self { frames }
    }
}
