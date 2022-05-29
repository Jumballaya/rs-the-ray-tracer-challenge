mod draw;
mod math;

use draw::{canvas::*, color::Color};

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(5, 3);

    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);

    c.set_pixel((0, 0), &c1);

    c.set_pixel((2, 1), &c2);
    c.set_pixel((4, 2), &c3);

    c.save("./", "test")
}
