use std::collections::HashMap;

use crate::geometry::Matrix;

use super::obj_parser::parse_point_3;

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
        for line in file.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let line = line.to_string();
            let split: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
            if split.is_empty() {
                continue;
            }
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
}

fn parse_transform(data: &[&str]) -> Matrix {
    if data.len() < 3 || data.len() % 3 != 0 {
        panic!("Incorrect transform for a bone");
    }

    log::info!("parse_transform data: {:?}", &data[0..3]);
    let angles = parse_point_3(&data[0..3]);

    Matrix::rotation_x(angles.x()) * Matrix::rotation_y(angles.y()) * Matrix::rotation_z(angles.z())
}
