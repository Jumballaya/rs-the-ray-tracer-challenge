use raytracer::{
    draw::{canvas::Canvas, color::Color},
    math::{point::Point, ray::Ray, tuple::Tuple},
    render::{intersections::Intersections, object::Object},
};

const SIZE: usize = 100;
const WALL: f64 = 7.0;

fn main() -> std::io::Result<()> {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let mut canvas = Canvas::new(SIZE, SIZE);
    let color = Color::red();
    let obj = Object::new_sphere();
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

            if let Some(_) = intersections.get_hit() {
                canvas.set_pixel((x, y), &color);
            }
        }
    }

    canvas.save("./", "chapter5")
}
