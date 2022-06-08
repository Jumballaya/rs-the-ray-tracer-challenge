pub mod math {
    pub mod epsilon;
    pub mod matrix;
    pub mod point;
    pub mod ray;
    pub mod transformation;
    pub mod tuple;
    pub mod vector;
}

pub mod draw {
    pub mod canvas;
    pub mod color;
}

pub mod render {
    pub mod camera;
    pub mod intersections;
    pub mod light;
    pub mod material;
    pub mod object;
    pub mod shape;
    pub mod world;

    pub mod pattern;
    pub mod patterns;

    pub mod shapes {
        pub mod plane;
        pub mod sphere;
        pub mod test_shape;
    }

    pub mod lights {
        pub mod point_light;
    }
}
