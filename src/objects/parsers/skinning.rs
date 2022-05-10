use crate::{geometry::vector::Vector4, objects::parsers::parse_point_2};

use super::obj_lines;

#[derive(Debug)]
pub struct Skinning {
    pub vertices: Vec<SkinningVertex>,
}

#[derive(Debug)]
pub struct SkinningVertex {
    pub bones: Vector4,
    pub weights: Vector4,
}

impl Skinning {
    pub fn parse(file: &str) -> Self {
        let mut skin = Self { vertices: vec![] };

        for split in obj_lines(file) {
            match split[0] {
                "vl" => skin
                    .vertices
                    .push(Self::parse_skinning_vertices(&split[2..])),
                _ => continue,
            }
        }

        skin
    }

    fn parse_skinning_vertices(split: &[&str]) -> SkinningVertex {
        if split.len() % 2 != 0 || split.len() > 8 {
            panic!("Invalid skinning vertex {:?}", split);
        }

        let mut bones = Vector4::from_xyzw(0.0, 0.0, 0.0, 0.0);
        let mut weights = Vector4::from_xyzw(0.0, 0.0, 0.0, 0.0);

        for i in 0..(split.len() / 2) {
            let pair = parse_point_2(&split[(2 * i)..(2 * i + 2)]);
            bones.set(i, pair.x);
            weights.set(i, pair.y);
        }

        log::info!("Skinning parse_skinning_vertices bones = {:?} weights = {:?}", bones, weights);

        SkinningVertex { bones, weights }
    }
}
