use raytracer::{
    draw::{canvas::Canvas, color::Color},
    math::{point::Point, tuple::Tuple, vector::Vector},
};

type Environment = (Vector, Vector);
type Projectile = (Point, Vector);

fn tick((gravity, wind): &Environment, (pos, vel): Projectile) -> Projectile {
    let position = pos + vel;
    let velocity = vel + *gravity + *wind;
    (position, velocity)
}

fn in_bounds((pos, _): &Projectile, x: f64, y: f64) -> bool {
    pos.y() >= 0.0 && pos.y() <= y && pos.x() >= 0.0 && pos.x() <= x
}

fn main() -> std::io::Result<()> {
    let proj_start = Point::new(0.0, 1.0, 0.0);
    let proj_velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let color = Color::new(1.0, 0.0, 0.0);

    let env: Environment = (gravity, wind);
    let mut proj: Projectile = (proj_start, proj_velocity);
    let mut canvas = Canvas::new(900, 550);

    while in_bounds(&proj, 900.0, 550.0) {
        let x = proj.0.x().round() as usize;
        let y = (550.0 - proj.0.y()).max(0.0).round() as usize; // must flip y axis because y=0 is at the top
        canvas.set_pixel((x, y), &color);
        proj = tick(&env, proj);
    }

    canvas.save("./", "chapter2")
}
