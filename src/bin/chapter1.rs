use raytracer::math::{point::Point, tuple::Tuple, vector::Vector};

type Environment = (Vector, Vector);
type Projectile = (Point, Vector);

fn tick((gravity, wind): &Environment, (pos, vel): Projectile) -> Projectile {
    let position = pos + vel;
    let velocity = vel + *gravity + *wind;
    (position, velocity)
}

fn in_bounds((pos, _): &Projectile) -> bool {
    pos.y() >= 0.0 && pos.y() <= 100.0 && pos.x() >= 0.0 && pos.x() <= 100.0
}

fn main() {
    let env: Environment = (Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let mut proj: Projectile = (
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize(),
    );

    while in_bounds(&proj) {
        println!("{}", proj.0.y());
        proj = tick(&env, proj);
    }
}
