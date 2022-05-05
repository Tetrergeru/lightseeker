use crate::geometry::{Vector2, Vector3, transform::RawTransform};

pub mod shape;
pub mod skeleton;
pub mod skinning;

pub fn parse_point_3(split: &[&str]) -> Vector3 {
    if split.len() < 3 {
        panic!("Not enough coordinates for a point 3");
    }

    let mut coords = [0.0; 3];
    for i in 0..3 {
        coords[i] = split[i].parse().unwrap();
    }

    Vector3::from_xyz(coords[0], coords[1], coords[2])
}

pub fn parse_point_2(split: &[&str]) -> Vector2 {
    if split.len() < 2 {
        panic!("Not enough coordinates for a point 3");
    }

    let mut coords = [0.0; 2];
    for i in 0..2 {
        coords[i] = split[i].parse().unwrap();
    }

    Vector2::from_xy(coords[0], 1.0 - coords[1])
}

pub fn parse_transform(data: &[&str]) -> RawTransform {
    if data.len() < 3 || data.len() % 3 != 0 {
        panic!("Incorrect transform for a bone");
    }

    log::info!("parse_transform data: {:?}", &data[0..3]);
    let angles = parse_point_3(&data[0..3]);

    let mut t = RawTransform::new();
    t.rotate(angles);
    t
}

pub fn obj_lines(file: &str) -> impl Iterator<Item = Vec<&str>> {
    file.lines()
        .filter(|it| !it.is_empty() && !it.starts_with('#'))
        .map(|it| it.split(' ').filter(|s| !s.is_empty()).collect())
        .filter(|it: &Vec<&str>| !it.is_empty())
}
