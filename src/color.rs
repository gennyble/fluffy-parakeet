// I write this struct so many times ahhhhhhhhhhhhhh
#[derive(Copy, Clone, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Rgba {
    fn from(c: (f32, f32, f32, f32)) -> Self {
        Self {
            r: c.0,
            g: c.1,
            b: c.2,
            a: c.3,
        }
    }
}

impl From<(f32, f32, f32)> for Rgba {
    fn from(c: (f32, f32, f32)) -> Self {
        Self {
            r: c.0,
            g: c.1,
            b: c.2,
            a: 1.0,
        }
    }
}

impl From<Rgb> for Rgba {
    fn from(c: Rgb) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<(f32, f32, f32)> for Rgb {
    fn from(c: (f32, f32, f32)) -> Self {
        Self {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
}
