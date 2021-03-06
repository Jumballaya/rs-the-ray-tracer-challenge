use std::fmt::Display;
use std::ops;

use crate::math::epsilon::ApproxEq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn scale(&self) -> (u8, u8, u8) {
        let red = ((self.r * 255.0) as u8).max(0).min(255);
        let green = ((self.g * 255.0) as u8).max(0).min(255);
        let blue = ((self.b * 255.0) as u8).max(0).min(255);
        (red, green, blue)
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {}, {})", self.r, self.g, self.b).as_str())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq(other.r) && self.b.approx_eq(other.b) && self.g.approx_eq(other.g)
    }

    fn ne(&self, other: &Self) -> bool {
        !(self.r.approx_eq(other.r) && self.b.approx_eq(other.b) && self.g.approx_eq(other.g))
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::epsilon::ApproxEq;

    use super::Color;

    #[test]
    fn color_can_create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(c.r.approx_eq(-0.5));
        assert!(c.g.approx_eq(0.4));
        assert!(c.b.approx_eq(1.7));
    }

    #[test]
    fn color_can_scale_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        let want: (u8, u8, u8) = (0, 102, 255);
        assert_eq!(c.scale(), want);
    }

    #[test]
    fn color_can_add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let want = Color::new(1.6, 0.7, 1.0);
        let got = c1 + c2;
        assert_eq!(got, want);
    }

    #[test]
    fn color_can_subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let want = Color::new(0.2, 0.5, 0.5);
        let got = c1 - c2;
        assert_eq!(got, want);
    }

    #[test]
    fn color_can_multiply_color_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let c2 = Color::new(0.2, 0.3, 0.4);
        let scalar1 = 2.0;
        let scalar2 = 4.0;
        let want1 = Color::new(0.4, 0.6, 0.8);
        let want2 = Color::new(0.8, 1.2, 1.6);
        let got1 = c1 * scalar1;
        let got2 = scalar2 * c2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn color_can_multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let want = Color::new(0.9, 0.2, 0.04);
        let got = c1 * c2;
        assert_eq!(got, want);
    }
}
