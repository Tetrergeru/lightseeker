use crate::vector::{Vector2, Vector3};

use super::vertex_data::VertexData;

pub struct ObjParser {
    points: Vec<Vector3>,
    texture_coords: Vec<Vector2>,
    normals: Vec<Vector3>,
    vertices: Vec<VertexData>,
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
        let base_vertex = self.parse_vertex_data(split[1]);

        for i in 2..split.len() - 1 {
            self.vertices.push(base_vertex.clone());
            self.vertices.push(self.parse_vertex_data(split[i]));
            self.vertices.push(self.parse_vertex_data(split[i + 1]));
        }
    }

    fn parse_vertex_data(&self, string: &str) -> VertexData {
        let split: Vec<&str> = string.split('/').collect();

        let point_idx: usize = split[0].parse().unwrap();
        let point = self.points[point_idx - 1];

        let texture_idx: usize = split[1].parse().unwrap();
        let texture = self.texture_coords[texture_idx - 1];

        let normal_idx: usize = split[2].parse().unwrap();
        let normal = self.normals[normal_idx - 1];

        VertexData {
            point,
            texture_coord: texture,
            normal,
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
