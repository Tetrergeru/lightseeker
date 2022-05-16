use std::{cell::RefCell, collections::HashMap};

use crate::geometry::{transform::RawTransform, Matrix, Transform};

use super::{obj_lines, parse_transform};

#[derive(Debug)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub names: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Bone {
    pub parent: isize,
    pub name: String,
    pub initial_transform: RawTransform,
}

pub struct BoneTransform {
    pub transform: Transform,
    pub initial: RawTransform,
    pub posed: RefCell<RawTransform>,
}

impl BoneTransform {
    pub fn matrix(&self) -> Matrix {
        self.posed.borrow().matrix() * self.initial.reverse_matrix()
    }

    pub fn set_pose(&self, pose: RawTransform) {
        *self.posed.borrow_mut() = pose;
        self.transform.set_transform(pose);
    }
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
                        initial_transform: RawTransform::new(),
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

    pub fn make_nested_transforms(&self, parent: Transform) -> Vec<BoneTransform> {
        let mut transforms = Vec::with_capacity(self.bones.len());

        for bone in self.bones.iter() {
            transforms.push(BoneTransform {
                initial: bone.initial_transform,
                posed: RefCell::new(bone.initial_transform),
                transform: Transform::from_raw(bone.initial_transform),
            });
        }

        for (idx, _bone) in self.bones.iter().enumerate() {
            transforms[idx].transform.set_parent(parent.clone());
        }

        transforms
    }
}
