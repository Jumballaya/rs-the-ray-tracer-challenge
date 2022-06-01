mod draw;
mod math;
mod render;

use draw::{canvas::*, color::*};
use math::{ray::Ray, tuple::*};
use render::{
    hit::{Hittable, Intersection},
    object::sphere::Sphere,
};

pub fn draw(width: usize, height: usize, canvas: &mut Canvas) {
    let origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / (width as f64);
    let half = wall_size / 2.0;
    let color = Color::new(1.0, 0.0, 0.0);
    let sphere = Sphere::new();

    for y in 0..height {
        let world_y = half - (pixel_size * (y as f64));
        for x in 0..width {
            let world_x = -half + (pixel_size * (x as f64));
            let pos = Tuple::new_point(world_x, world_y, wall_z);
            let ray = {
                let o_t = origin.as_tuple();
                let o = (o_t.1, o_t.2, o_t.3);
                let d_t = (pos - origin).normalize().as_tuple();
                let d = (d_t.1, d_t.2, d_t.3);
                Ray::new(o, d)
            };
            let s = sphere.clone();
            let intersections = s.intersect(ray);
            if let Some(_) = Intersection::get_hit(&intersections) {
                canvas.set_pixel((x, y), &color);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(500, 500);
    draw(500, 500, &mut c);
    c.save("./", "test")
}
