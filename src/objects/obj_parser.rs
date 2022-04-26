use crate::vector::{Vector2, Vector3};

use super::vertex_data::VertexData;

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
}

impl RawVertexData {
    fn expect_some(self) -> VertexData {
        VertexData {
            point: self.point,
            texture_coord: self.texture_coord,
            normal: self.normal.unwrap(),
        }
    }

    fn expect_none(self, normal: Vector3) -> VertexData {
        VertexData {
            point: self.point,
            texture_coord: self.texture_coord,
            normal,
        }
    }
}

impl ObjParser {
    pub fn parse(file: &str) -> Vec<VertexData> {
        Self::new().parse_obj(file)
    }

    fn new() -> Self {
        Self {
            points: vec![],
            texture_coords: vec![],
            normals: vec![],
            vertices: vec![],
        }
    }

    fn parse_obj(mut self, file: &str) -> Vec<VertexData> {
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
                "v" => self.points.push(parse_point_3(&split)),
                "vt" => self.texture_coords.push(parse_point_2(&split)),
                "vn" => self.normals.push(parse_point_3(&split)),
                "f" => self.parse_polygon(&split),
                _ => continue,
            }
        }
        self.vertices
    }

    fn parse_polygon(&mut self, split: &[&str]) {
        if split.len() < 3 {
            panic!("Not enough entities for a polygon");
        }
        let v0 = self.parse_vertex_data(split[1]);
        let missing_normales = v0.normal.is_none();

        for i in 2..split.len() - 1 {
            let v1 = self.parse_vertex_data(split[i]);
            let v2 = self.parse_vertex_data(split[i + 1]);
            if !missing_normales {
                self.vertices.push(v0.expect_some());
                self.vertices.push(v1.expect_some());
                self.vertices.push(v2.expect_some());
            } else {
                let normal = (v1.point - v0.point).cross(v2.point - v0.point);
                self.vertices.push(v0.expect_none(normal));
                self.vertices.push(v1.expect_none(normal));
                self.vertices.push(v2.expect_none(normal));
            }
        }
    }

    fn parse_vertex_data(&self, string: &str) -> RawVertexData {
        let split: Vec<&str> = string.split('/').collect();

        let point_idx: usize = split[0].parse().unwrap();
        let point = self.points[point_idx - 1];

        let texture_idx: usize = split[1].parse().unwrap();
        let texture = self.texture_coords[texture_idx - 1];

        if split.len() < 3 {
            RawVertexData {
                point,
                texture_coord: texture,
                normal: None,
            }
        } else {
            let normal_idx: usize = split[2].parse().unwrap();
            let normal = Some(self.normals[normal_idx - 1]);

            RawVertexData {
                point,
                texture_coord: texture,
                normal,
            }
        }
    }
}

fn parse_point_3(split: &[&str]) -> Vector3 {
    if split.len() < 4 {
        panic!("Not enough coordinates for a point 3");
    }

    let mut coords = [0.0; 3];
    for i in 0..3 {
        coords[i] = split[i + 1].parse().unwrap();
    }

    Vector3::from_xyz(coords[0], coords[1], coords[2])
}

fn parse_point_2(split: &[&str]) -> Vector2 {
    if split.len() < 3 {
        panic!("Not enough coordinates for a point 3");
    }

    let mut coords = [0.0; 2];
    for i in 0..2 {
        coords[i] = split[i + 1].parse().unwrap();
    }

    Vector2::from_xy(coords[0], 1.0 - coords[1])
}
