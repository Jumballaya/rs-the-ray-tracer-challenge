use std::f64::consts::PI;

use raytracer::{
    draw::{canvas::Canvas, color::Color},
    math::{point::Point, transformation::rotate_z, tuple::Tuple},
};

fn convert_pos(x: f64, y: f64) -> (usize, usize) {
    let new_y = if y < 50.0 {
        100 - (y.round() + 50.0) as usize
    } else {
        100 - (y.round() - 50.0) as usize
    };
    let new_x = if x < 50.0 {
        (x.round() + 50.0) as usize
    } else {
        (x.round() - 50.0) as usize
    };
    (new_x, new_y)
}

fn draw_at(point: &Point, color: &Color, canvas: &mut Canvas) {
    canvas.set_pixel(convert_pos(point.x(), point.y()), color);
}

fn create_clock() -> Vec<Point> {
    let twelve = Point::new(0.0, 25.0, 0.0);
    (0..=12)
        .collect::<Vec<usize>>()
        .iter()
        .map(|t| rotate_z(*t as f64 * (-PI / 12.0)) * twelve)
        .chain(
            (0..=12)
                .collect::<Vec<usize>>()
                .iter()
                .map(|t| rotate_z(*t as f64 * (PI / 12.0)) * twelve),
        )
        .collect()
}

fn main() -> std::io::Result<()> {
    let center = Point::new(0.0, 0.0, 0.0);
    let clock = create_clock();
    let mut canvas = Canvas::new(100, 100);

    draw_at(&center, &Color::red(), &mut canvas);
    for point in clock {
        draw_at(&point, &Color::white(), &mut canvas);
    }

    canvas.save("./", "chapter4")
}
