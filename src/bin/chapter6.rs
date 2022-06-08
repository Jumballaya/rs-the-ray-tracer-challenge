use raytracer::{
    draw::{canvas::Canvas, color::Color},
    math::{point::Point, ray::Ray, tuple::Tuple},
    render::{
        intersections::Intersections, light::Light, lights::point_light::PointLight,
        material::Materialable, object::Object, pattern::Pattern,
    },
};

const SIZE: usize = 100;
const WALL: f64 = 7.0;

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(SIZE, SIZE);

    let obj = Object::new_sphere().with_pattern(Pattern::new_solid(Color::new(1.0, 0.2, 1.0)));
    let light = Light::Point(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::white(),
    ));

    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let pixel_size = WALL / (SIZE as f64);
    let half = WALL / 2.0;

    for y in 0..100 {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..100 {
            let world_x = -half + pixel_size * (x as f64);
            let pos = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (pos - origin).normalize());
            let mut intersections = Intersections::new();
            obj.intersect(&ray, &mut intersections);

            if let Some(hit) = intersections.get_hit() {
                let point = ray.position_at(hit.t());
                let eye = -ray.direction;
                let normal = hit.object().normal_at(&point);
                let color = light.lighting(
                    hit.object(),
                    &hit.object().get_material(),
                    point,
                    eye,
                    normal,
                    false,
                );
                canvas.set_pixel((x, y), &color);
            }
        }
    }

    canvas.save("./", "chapter6")
}
