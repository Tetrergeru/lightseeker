#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_float(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self::new(f64_to_u8(r), f64_to_u8(g), f64_to_u8(b), f64_to_u8(a))
    }

    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        let h = h / (std::f64::consts::PI / 3.0);

        let c = v * s;

        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());

        let rgb = if (0.0..1.0).contains(&h) {
            (c, x, 0.0)
        } else if (1.0..2.0).contains(&h) {
            (x, c, 0.0)
        } else if (2.0..3.0).contains(&h) {
            (0.0, c, x)
        } else if (3.0..4.0).contains(&h) {
            (0.0, x, c)
        } else if (4.0..5.0).contains(&h) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let m = v - c;

        Self::from_float(rgb.0 + m, rgb.1 + m, rgb.2 + m, 1.0)
    }

    pub fn to_style(self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    pub fn get_r(self) -> u8 {
        self.r
    }

    pub fn get_g(self) -> u8 {
        self.g
    }

    pub fn get_b(self) -> u8 {
        self.b
    }

    pub const BLACK: Color = Self::new(0, 0, 0, 255);
    pub const WHITE: Color = Self::new(255, 255, 255, 255);
    pub const RED: Color = Self::new(255, 0, 0, 255);
}

fn f64_to_u8(v: f64) -> u8 {
    let v = (v * 255.0) as isize;
    if v > 255 {
        255
    } else if v < 0 {
        0
    } else {
        v as u8
    }
}
