pub struct Matrix {
    pub matrix: [[f64; 4]; 4],
}

impl Matrix {
    fn new() -> Self {
        Self {
            matrix: [[0.0; 4]; 4],
        }
    }
}
