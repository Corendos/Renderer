
#[derive(Clone, Copy)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl Color<f32> {
    pub const RED: Color<f32> = Color::<f32> {r: 1.0, g: 0.0, b: 0.0};
    pub const GREEN: Color<f32> = Color::<f32> {r: 0.0, g: 1.0, b: 0.0};
    pub const BLUE: Color<f32> = Color::<f32> {r: 0.0, g: 0.0, b: 1.0};
    pub const CYAN: Color<f32> = Color::<f32> {r: 0.0, g: 1.0, b: 1.0};
    pub const MAGENTA: Color<f32> = Color::<f32> {r: 1.0, g: 0.0, b: 1.0};
    pub const YELLOW: Color<f32> = Color::<f32> {r: 1.0, g: 1.0, b: 0.0};
    pub const BLACK: Color<f32> = Color::<f32> {r: 0.0, g: 0.0, b: 0.0};
    pub const WHITE: Color<f32> = Color::<f32> {r: 1.0, g: 1.0, b: 1.0};
}

impl std::ops::Mul<f32> for Color<f32> {
    type Output = Color<f32>;
    fn mul(self, lhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * lhs,
            g: self.g * lhs,
            b: self.b * lhs,
        }
    }
}

impl std::ops::Mul<Color<f32>> for f32 {
    type Output = Color<f32>;
    fn mul(self, lhs: Color<f32>) -> Self::Output {
        lhs * self
    }
}