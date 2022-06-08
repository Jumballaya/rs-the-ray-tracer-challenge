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
