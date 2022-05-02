use std::ops::{Deref, Mul};

use super::vector::{Vector3, Vector4};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub matrix: [f32; 16],
}

impl Matrix {
    pub const SIZE: usize = 4;

    pub fn zero() -> Self {
        Self {
            matrix: [0.0; Self::SIZE * Self::SIZE],
        }
    }

    pub fn ident() -> Self {
        let mut zero = Self::zero();
        for i in 0..Self::SIZE {
            zero.set(i, i, 1.0);
        }
        zero
    }

    pub fn translate(vector: Vector3) -> Self {
        let mut matrix = Self::ident();
        for i in 0..(Self::SIZE - 1) {
            matrix.set(i, 3, vector.get(i));
        }
        matrix
    }

    pub fn scale(factor: f32) -> Self {
        let mut matrix = Self::ident();
        for i in 0..(Self::SIZE - 1) {
            matrix.set(i, i, factor);
        }
        matrix
    }

    pub fn perspective(field_of_view_in_radians: f32, aspect: f32, near: f32, far: f32) -> Matrix {
        let f = (std::f32::consts::PI * 0.5 - 0.5 * field_of_view_in_radians).tan();
        let range_inv = 1.0 / (near - far);

        let matrix: Self = [
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (near + far) * range_inv, -1.0],
            [0.0, 0.0, near * far * range_inv * 2.0, 0.0],
        ]
        .into();
        matrix.transpose()
    }

    pub fn rotation_x(a: f32) -> Matrix {
        let cos = a.cos();
        let sin = a.sin();
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn rotation_y(a: f32) -> Matrix {
        let cos = a.cos();
        let sin = a.sin();
        [
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self.matrix[i * Self::SIZE + j] = value;
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.matrix[i * Self::SIZE + j]
    }

    pub fn transpose(self) -> Self {
        let mut matrix = Self::zero();
        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                matrix.set(i, j, self.get(j, i));
            }
        }
        matrix
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::zero();

        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                let mut sum = 0.0;
                for k in 0..Self::SIZE {
                    sum += self.get(i, k) * rhs.get(k, j);
                }
                result.set(i, j, sum);
            }
        }

        result
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let mut result = Vector4::zero();

        for i in 0..Self::SIZE {
            let mut sum = 0.0;
            for k in 0..Self::SIZE {
                sum += self.get(i, k) * rhs.get(k);
            }
            result.set(i, sum);
        }

        result
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let mut result = Vector3::zero();

        for i in 0..3 {
            let mut sum = 0.0;
            for k in 0..3 {
                sum += self.get(i, k) * rhs.get(k);
            }
            result.set(i, sum + self.get(i, 3));
        }

        result
    }
}

impl Deref for Matrix {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl From<[[f32; Matrix::SIZE]; Matrix::SIZE]> for Matrix {
    fn from(arr: [[f32; Matrix::SIZE]; Matrix::SIZE]) -> Self {
        let mut matrix = Matrix::zero();
        for (i, item) in arr.iter().enumerate() {
            for (j, item) in item.iter().enumerate() {
                matrix.set(i, j, *item);
            }
        }
        matrix
    }
}
