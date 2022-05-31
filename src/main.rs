mod draw;
mod math;
mod render;

use draw::{canvas::*, color::*};
use math::tuple::*;

pub fn draw(width: usize, height: usize, canvas: &mut Canvas) {
    let mut pos = Tuple::new_vector(0.0, 1.0, 0.0);
    let mut vel = Tuple::new_vector(0.1, 0.2, 0.0).normalize() * 5.25;
    let color = Color::new(1.0, 0.0, 0.0);
    let wind = Tuple::new_vector(-0.001, 0.0, 0.0);
    let gravity = Tuple::new_vector(0.00, -0.02, 0.0);

    while (pos.x as usize) < width && (pos.y as usize) < height && pos.y > 0.0 {
        canvas.set_pixel((pos.x as usize, height - (pos.y as usize)), &color);
        vel = vel + gravity + wind;
        pos = pos + vel;
    }
}

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(1_000, 1_000);
    draw(1_000, 1_000, &mut c);
    c.save("./", "test")
}
