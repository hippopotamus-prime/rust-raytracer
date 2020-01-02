use std::ops;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn clamp(&mut self) {
        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.b > 1.0 {
            self.b = 1.0;
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Add<Color> for &Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Add<&Color> for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        Color {
            r: self.r + rhs.r, 
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, other: &Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scale: f32) -> Color {
        Color {
            r: self.r * scale,
            g: self.g * scale,
            b: self.b * scale
        }
    }
}

impl ops::Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, scale: f32) -> Color {
        Color {
            r: self.r * scale,
            g: self.g * scale,
            b: self.b * scale
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: color.r * self,
            g: color.g * self,
            b: color.b * self
        }
    }
}

impl ops::Mul<&Color> for f32 {
    type Output = Color;

    fn mul(self, color: &Color) -> Color {
        Color {
            r: color.r * self,
            g: color.g * self,
            b: color.b * self
        }
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, scale: f32) {
        self.r *= scale;
        self.g *= scale;
        self.b *= scale;
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, scale: f32) -> Color {
        Color {
            r: self.r / scale,
            g: self.g / scale, 
            b: self.b / scale
        }
    }
}

impl ops::Div<f32> for &Color {
    type Output = Color;

    fn div(self, scale: f32) -> Color {
        Color {
            r: self.r / scale,
            g: self.g / scale, 
            b: self.b / scale
        }
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, scale: f32) {
        self.r /= scale;
        self.g /= scale;
        self.b /= scale;
    }
}
