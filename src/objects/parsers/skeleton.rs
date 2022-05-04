use std::collections::HashMap;

use crate::{
    geometry::{Matrix, Transform},
    objects::parsers::parse_point_3,
};

use super::{obj_lines, parse_transform};

#[derive(Debug)]
pub struct Skeleton {
    bones: Vec<Bone>,
    names: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Bone {
    pub parent: isize,
    pub name: String,
    pub initial_transform: Matrix,
}

impl Skeleton {
    pub fn from_file(file: &str) -> Self {
        let mut skl = Self {
            bones: vec![],
            names: HashMap::new(),
        };
        for split in obj_lines(file) {
            match split[0] {
                "b" => {
                    skl.bones.push(Bone {
                        name: split[1].to_string(),
                        parent: -1,
                        initial_transform: Matrix::ident(),
                    });
                    skl.names.insert(split[1].to_string(), skl.bones.len() - 1);
                }
                "bp" => skl.bones.last_mut().unwrap().parent = skl.names[split[1]] as isize,
                "bb" => {
                    skl.bones.last_mut().unwrap().initial_transform = parse_transform(&split[1..]);
                }
                _ => continue,
            }
        }
        skl
    }

    pub fn make_nested_transforms(&self) -> Vec<Transform> {
        let mut transforms = Vec::with_capacity(self.bones.len());

        for bone in self.bones.iter() {
            transforms.push(Transform::new());
        }

        transforms
    }
}
