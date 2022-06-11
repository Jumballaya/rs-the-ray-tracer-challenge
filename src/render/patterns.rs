use crate::{
    draw::color::Color,
    math::{epsilon::ApproxEq, point::Point, tuple::Tuple},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn pattern_at(&self, _: &Point) -> Color {
        self.color
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        if (point.x().floor().abs() as usize) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn new() -> Self {
        Self {}
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        self.a + point.x() * (self.b - self.a)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let distance = ((point.x() * point.x()) + (point.z() * point.z())).sqrt();
        if (distance.floor() as usize) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CheckerPattern {
    a: Color,
    b: Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let sum = point.x().floor() + point.y().floor() + point.z().floor();
        if (sum % 2.0).approx_eq(0.0) {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NoisePattern {
    color: Color,
}

static perlin_permutation: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

impl NoisePattern {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let n = NoisePattern::perlin(point.x(), point.y(), point.z());
        self.color * n
    }

    pub fn perlin(x: f64, y: f64, z: f64) -> f64 {
        let x_perm = x.floor() as u8 & 255;
        let y_perm = y.floor() as u8 & 255;
        let z_perm = z.floor() as u8 & 255;

        let x = x - x.floor();
        let y = y - y.floor();
        let z = z - z.floor();

        let u = NoisePattern::fade(x);
        let v = NoisePattern::fade(y);
        let w = NoisePattern::fade(z);

        let a = perlin_permutation[x_perm as usize] + y_perm;
        let aa = perlin_permutation[a as usize] + z_perm;
        let ab = perlin_permutation[(a + 1) as usize] + z_perm;
        let b = perlin_permutation[(x_perm + 1) as usize] + y_perm;
        let ba = perlin_permutation[b as usize] + z_perm;
        let bb = perlin_permutation[(b + 1) as usize] + z_perm;

        let from = NoisePattern::lerp(
            v,
            NoisePattern::grad(perlin_permutation[aa as usize], x, y, z),
            NoisePattern::grad(perlin_permutation[ba as usize], x - 1.0, y, z),
        );

        let to = NoisePattern::lerp(
            v,
            NoisePattern::lerp(
                u,
                NoisePattern::grad(perlin_permutation[(aa + 1) as usize], x, y, z - 1.0),
                NoisePattern::grad(perlin_permutation[(ba + 1) as usize], x - 1.0, y, z - 1.0),
            ),
            NoisePattern::lerp(
                u,
                NoisePattern::grad(perlin_permutation[(ab + 1) as usize], x, y - 1.0, z - 1.0),
                NoisePattern::grad(
                    perlin_permutation[(bb + 1) as usize],
                    x - 1.0,
                    y - 1.0,
                    z - 1.0,
                ),
            ),
        );

        NoisePattern::scale(NoisePattern::lerp(w, from, to))
    }

    fn lerp(t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn scale(n: f64) -> f64 {
        (1.0 + n) / 2.0
    }

    fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        let l = if h & 1 == 0 { u } else { -u };
        let r = if h & 2 == 0 { v } else { -v };
        l + r
    }
}

#[cfg(test)]
mod test {
    use super::{CheckerPattern, GradientPattern, RingPattern, StripePattern};

    use crate::draw::color::Color;
    use crate::math::{point::Point, tuple::Tuple};

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pat = StripePattern {
            a: Color::white(),
            b: Color::black(),
        };
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pat = StripePattern {
            a: Color::white(),
            b: Color::black(),
        };
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pat = StripePattern {
            a: Color::white(),
            b: Color::black(),
        };
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.pattern_at(&Point::new(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pat.pattern_at(&Point::new(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.pattern_at(&Point::new(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn gradient_pattern_linearly_interpolates_between_colors() {
        let pat = GradientPattern {
            a: Color::white(),
            b: Color::black(),
        };

        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(
            pat.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pat.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pat.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_pattern_should_extend_in_both_x_and_z() {
        let pat = RingPattern {
            a: Color::white(),
            b: Color::black(),
        };

        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(01.0, 0.0, 0.0)), Color::black());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(
            pat.pattern_at(&Point::new(0.708, 0.0, 0.708)),
            Color::black()
        );
    }

    #[test]
    fn checker_pattern_should_repeat_in_x() {
        let pat = CheckerPattern {
            a: Color::white(),
            b: Color::black(),
        };

        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checker_pattern_should_repeat_in_y() {
        let pat = CheckerPattern {
            a: Color::white(),
            b: Color::black(),
        };

        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checker_pattern_should_repeat_in_z() {
        let pat = CheckerPattern {
            a: Color::white(),
            b: Color::black(),
        };

        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pat.pattern_at(&Point::new(0.0, 0.0, 1.01)), Color::black());
    }
}
