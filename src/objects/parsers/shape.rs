use crate::geometry::{vector::Vector4, Vector2, Vector3};

use super::{obj_lines, parse_point_2, parse_point_3, skinning::Skinning};

#[derive(Clone, Debug)]
pub struct VertexData {
    pub point: Vector3,
    pub normal: Vector3,
    pub texture_coord: Vector2,
    pub bones: Vector4,
    pub weights: Vector4,
}

pub struct ObjParser {
    points: Vec<Vector3>,
    texture_coords: Vec<Vector2>,
    normals: Vec<Vector3>,
    vertices: Vec<VertexData>,
}

#[derive(Clone, Copy)]
struct RawVertexData {
    pub point: Vector3,
    pub normal: Option<Vector3>,
    pub texture_coord: Vector2,
    pub bones: Option<Vector4>,
    pub weights: Option<Vector4>,
}

impl RawVertexData {
    fn unwrap(self) -> VertexData {
        VertexData {
            point: self.point,
            texture_coord: self.texture_coord,
            normal: self.normal.unwrap(),
            bones: self.bones.unwrap_or_default(),
            weights: self.weights.unwrap_or_default(),
        }
    }

    fn with_normals(mut self, normal: Vector3) -> RawVertexData {
        self.normal = Some(normal);
        self
    }
}

impl ObjParser {
    pub fn parse(file: &str) -> Vec<VertexData> {
        Self::new().parse_obj(file, None)
    }

    pub fn parse_with_skin(file: &str, skin: &Skinning) -> Vec<VertexData> {
        Self::new().parse_obj(file, Some(skin))
    }

    fn new() -> Self {
        Self {
            points: vec![],
            texture_coords: vec![],
            normals: vec![],
            vertices: vec![],
        }
    }

    fn parse_obj(mut self, file: &str, skinning: Option<&Skinning>) -> Vec<VertexData> {
        for split in obj_lines(file) {
            match split[0] {
                "v" => self.points.push(parse_point_3(&split[1..])),
                "vt" => {
                    let coord = parse_point_2(&split[1..]);
                    self.texture_coords
                        .push(Vector2::from_xy(coord.x(), 1.0 - coord.y()));
                }
                "vn" => self.normals.push(parse_point_3(&split[1..])),
                "f" => self.parse_polygon(&split[1..], skinning),
                _ => continue,
            }
        }
        self.vertices
    }

    fn parse_polygon(&mut self, split: &[&str], skinning: Option<&Skinning>) {
        if split.len() < 2 {
            panic!("Not enough entities for a polygon");
        }
        let v0 = self.parse_vertex_data(split[0], skinning);
        let missing_normals = v0.normal.is_none();

        for i in 1..split.len() - 1 {
            let v1 = self.parse_vertex_data(split[i], skinning);
            let v2 = self.parse_vertex_data(split[i + 1], skinning);
            if !missing_normals {
                self.vertices.push(v0.unwrap());
                self.vertices.push(v1.unwrap());
                self.vertices.push(v2.unwrap());
            } else {
                let normal = (v1.point - v0.point).cross(v2.point - v0.point);
                self.vertices.push(v0.with_normals(normal).unwrap());
                self.vertices.push(v1.with_normals(normal).unwrap());
                self.vertices.push(v2.with_normals(normal).unwrap());
            }
        }
    }

    fn parse_vertex_data(&self, string: &str, skinning: Option<&Skinning>) -> RawVertexData {
        let split: Vec<&str> = string.split('/').collect();

        let point_idx = split[0].parse::<usize>().unwrap() - 1;
        let point = self.points[point_idx];

        let texture_idx = split[1].parse::<usize>().unwrap() - 1;
        let texture = self.texture_coords[texture_idx];

        let skinning_vertex = skinning.map(|it| &it.vertices[point_idx]);

        RawVertexData {
            point,
            texture_coord: texture,
            normal: if split.len() < 3 {
                None
            } else {
                let normal_idx: usize = split[2].parse().unwrap();
                Some(self.normals[normal_idx - 1])
            },
            bones: skinning_vertex.map(|it| it.bones),
            weights: skinning_vertex.map(|it| it.weights),
        }
    }
}
