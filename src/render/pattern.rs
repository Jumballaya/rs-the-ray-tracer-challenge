use crate::{
    draw::color::Color,
    math::{matrix::Matrix, point::Point, transformation::Transformable},
};

use crate::render::patterns::{SolidPattern, StripePattern, TestPattern};

use super::{
    object::Object,
    patterns::{CheckerPattern, GradientPattern, RingPattern},
};

#[derive(Clone, Debug, Copy, PartialEq)]
enum PatternType {
    Stripe(StripePattern),
    Solid(SolidPattern),
    Test(TestPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternType,
    transformation: Matrix,
    inv_transform: Matrix,
}

impl Pattern {
    pub fn new_solid(color: Color) -> Self {
        Self {
            pattern: PatternType::Solid(SolidPattern::new(color)),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity().inverse(),
        }
    }

    pub fn new_stripe(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Stripe(StripePattern::new(a, b)),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn new_gradient(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Gradient(GradientPattern::new(a, b)),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn new_ring(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Ring(RingPattern::new(a, b)),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn new_checker(a: Color, b: Color) -> Self {
        Self {
            pattern: PatternType::Checker(CheckerPattern::new(a, b)),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        match &self.pattern {
            PatternType::Stripe(p) => p.pattern_at(point),
            PatternType::Solid(p) => p.pattern_at(point),
            PatternType::Test(p) => p.pattern_at(point),
            PatternType::Gradient(p) => p.pattern_at(point),
            PatternType::Ring(p) => p.pattern_at(point),
            PatternType::Checker(p) => p.pattern_at(point),
        }
    }

    #[allow(dead_code)]
    fn new_test() -> Self {
        Self {
            pattern: PatternType::Test(TestPattern::new()),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn pattern_at_object(&self, obj: &Object, world_point: &Point) -> Color {
        let obj_inv_tform = obj.get_transform_inv();
        let obj_point = obj_inv_tform * *world_point;
        let pattern_point = self.inv_transform * obj_point;
        self.pattern_at(&pattern_point)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            pattern: PatternType::Solid(SolidPattern::new(Color::white())),
            transformation: Matrix::identity(),
            inv_transform: Matrix::identity().inverse(),
        }
    }
}

impl Transformable for Pattern {
    fn with_transform(self, tform: Matrix) -> Self {
        let new_tform = tform * self.transformation;
        Self {
            pattern: self.pattern,
            transformation: new_tform,
            inv_transform: new_tform.inverse(),
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transformation
    }
}

#[cfg(test)]
mod test {

    use super::{Pattern, Transformable};
    use crate::{
        draw::color::Color,
        math::{matrix::Matrix, point::Point, transformation::translate, tuple::Tuple},
        render::{material::Materialable, object::Object},
    };

    #[test]
    fn stripes_with_an_object_transformation() {
        let obj = Object::new_sphere()
            .scale(2.0, 2.0, 2.0)
            .with_pattern(Pattern::new_stripe(Color::white(), Color::black()));
        let c = obj
            .get_material()
            .pattern
            .pattern_at_object(&obj, &Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let pat = Pattern::new_stripe(Color::white(), Color::black()).scale(2.0, 2.0, 2.0);
        let obj = Object::new_sphere().with_pattern(pat);
        let c = obj
            .get_material()
            .pattern
            .pattern_at_object(&obj, &Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_an_object_and_a_pattern_transformation() {
        let obj = Object::new_sphere().scale(2.0, 2.0, 2.0).with_pattern(
            Pattern::new_stripe(Color::white(), Color::black()).translate(0.5, 0.0, 0.0),
        );
        let c = obj
            .get_material()
            .pattern
            .pattern_at_object(&obj, &Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn default_pattern_transformation() {
        let pat = Pattern::new_test();
        assert_eq!(pat.get_transform(), Matrix::identity());
    }

    #[test]
    fn can_assign_a_transformation() {
        let pat = Pattern::new_test().translate(1.0, 2.0, 3.0);
        assert_eq!(pat.get_transform(), translate(1.0, 2.0, 3.0));
    }
}
