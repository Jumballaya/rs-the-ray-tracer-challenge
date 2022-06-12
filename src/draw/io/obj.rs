use std::collections::HashMap;

use crate::{
    math::{point::Point, tuple::Tuple, vector::Vector},
    render::{
        material::{Material, Materialable},
        object::Object,
    },
};

struct FaceVertex {
    vertex: usize,
    normal: Option<usize>,
}

struct Face {
    vertices: Vec<FaceVertex>,
    group: Option<String>,
}

impl Default for Face {
    fn default() -> Self {
        Self {
            vertices: vec![],
            group: None,
        }
    }
}

pub struct ObjFileParser {
    input: String,
    lines_ignored: usize,
    vertices: Vec<Point>,
    normals: Vec<Vector>,
    faces: Vec<Face>,
    current_group: Option<String>,
    material: Material,
}

impl ObjFileParser {
    pub fn new_file(path: &str) -> Self {
        let err_message = format!("Error reading OBJ file: {}", path);
        Self {
            input: std::fs::read_to_string(path).expect(&err_message),
            lines_ignored: 0,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
            current_group: None,
            material: Material::default(),
        }
    }

    pub fn new_input(input: String) -> Self {
        Self {
            input,
            lines_ignored: 0,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
            current_group: None,
            material: Material::default(),
        }
    }

    pub fn build(&mut self) -> Object {
        self.parse();

        let mut root_children = Vec::<Object>::new();
        let mut group_hash = HashMap::<String, Vec<Object>>::new();

        for face in &self.faces {
            let mut tris = self.fan_triangulation(&face.vertices);
            if let Some(grp) = &face.group {
                group_hash.insert(grp.clone(), tris);
            } else {
                root_children.append(&mut tris);
            }
        }

        let mut groups: Vec<Object> = group_hash
            .keys()
            .into_iter()
            .filter_map(|k| match group_hash.get(k) {
                Some(tris) => Some(Object::new_group(tris.clone())),
                None => None,
            })
            .collect();

        if groups.len() == 1 {
            match groups.first() {
                Some(g) => g.clone(),
                None => Object::new_group(vec![]),
            }
        } else {
            groups.append(&mut root_children);
            Object::new_group(groups)
        }
    }

    pub fn build_with_material(&mut self, mat: Material) -> Object {
        self.material = mat;
        self.build()
    }

    fn get_vertex(&self, index: usize) -> Point {
        self.vertices[(index - 1).max(0).min(self.vertices.len())]
    }

    fn parse(&mut self) {
        for line in self.input.lines() {
            let chars: Vec<char> = line.chars().collect();
            match chars.first() {
                Some('v') => match self.parse_vertex_line(line) {
                    Some(v) => self.vertices.push(v),
                    _ => self.lines_ignored += 1,
                },
                Some('f') => match self.parse_face_line(line) {
                    Some(f) => self.faces.push(f),
                    None => self.lines_ignored += 1,
                },
                Some('g') => {
                    let name: String = line.split(" ").skip(1).take(1).collect();
                    self.current_group = Some(name);
                }

                _ => {
                    self.lines_ignored += 1;
                }
            };
        }
    }

    fn parse_vertex_line(&self, line: &str) -> Option<Point> {
        let p_str: Vec<&str> = line.split(" ").skip(1).collect();
        if p_str.len() < 3 {
            None
        } else {
            let x = p_str[0].parse::<f64>().unwrap_or(0.0);
            let y = p_str[1].parse::<f64>().unwrap_or(0.0);
            let z = p_str[2].parse::<f64>().unwrap_or(0.0);

            Some(Point::new(x, y, z))
        }
    }

    fn parse_face_line(&self, line: &str) -> Option<Face> {
        let t_str: Vec<&str> = line.split(" ").skip(1).collect();
        if t_str.len() < 3 {
            None
        } else {
            let vertices: Vec<FaceVertex> = t_str
                .iter()
                .map(|s| s.parse::<usize>().unwrap_or(0))
                .map(|index| FaceVertex {
                    vertex: index,
                    normal: None,
                })
                .collect();
            Some(Face {
                vertices,
                group: self.current_group.clone(),
            })
        }
    }

    fn fan_triangulation(&self, vertices: &Vec<FaceVertex>) -> Vec<Object> {
        let mut tris = Vec::<Object>::new();

        for index in 1..(vertices.len() - 1) {
            let tri = Object::new_tri(
                self.get_vertex(vertices[0].vertex),
                self.get_vertex(vertices[index].vertex),
                self.get_vertex(vertices[index + 1].vertex),
            )
            .with_material(self.material);
            tris.push(tri);
        }

        tris
    }
}

#[cfg(test)]
mod test {
    use super::ObjFileParser;

    use crate::math::{point::Point, tuple::Tuple};

    #[test]
    fn ignore_unrecognized_lines() {
        let input = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";
        let mut parser = ObjFileParser::new_input(String::from(input));
        parser.parse();
        assert_eq!(parser.lines_ignored, 5);
    }

    #[test]
    fn vertex_records() {
        let input = "
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0";
        let inp = String::from(input);

        let mut parser = ObjFileParser::new_input(inp);
        parser.parse();

        assert_eq!(parser.vertices.len(), 4);
        assert_eq!(parser.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parser.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parser.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_eq!(parser.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_triangle_faces() {
        let input = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
";
        let inp = String::from(input);

        let mut parser = ObjFileParser::new_input(inp);

        let group = parser.build();
        let children = group.children().unwrap();
        let t1 = &children[0].get_shape().as_triangle().unwrap();
        let t2 = &children[1].get_shape().as_triangle().unwrap();

        assert_eq!(t1.p1(), parser.get_vertex(1));
        assert_eq!(t1.p2(), parser.get_vertex(2));
        assert_eq!(t1.p3(), parser.get_vertex(3));

        assert_eq!(t2.p1(), parser.get_vertex(1));
        assert_eq!(t2.p2(), parser.get_vertex(3));
        assert_eq!(t2.p3(), parser.get_vertex(4));
    }

    #[test]
    fn triangulating_polygons() {
        let input = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5
";
        let inp = String::from(input);

        let mut parser = ObjFileParser::new_input(inp);

        let group = parser.build();
        let children = group.children().unwrap();
        let t1 = &children[0].get_shape().as_triangle().unwrap();
        let t2 = &children[1].get_shape().as_triangle().unwrap();
        let t3 = &children[2].get_shape().as_triangle().unwrap();

        assert_eq!(t1.p1(), parser.get_vertex(1));
        assert_eq!(t1.p2(), parser.get_vertex(2));
        assert_eq!(t1.p3(), parser.get_vertex(3));

        assert_eq!(t2.p1(), parser.get_vertex(1));
        assert_eq!(t2.p2(), parser.get_vertex(3));
        assert_eq!(t2.p3(), parser.get_vertex(4));

        assert_eq!(t3.p1(), parser.get_vertex(1));
        assert_eq!(t3.p2(), parser.get_vertex(4));
        assert_eq!(t3.p3(), parser.get_vertex(5));
    }

    #[test]
    fn triangles_in_groups() {
        let input = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4";
        let inp = String::from(input);

        let mut parser = ObjFileParser::new_input(inp);

        let group = parser.build();
        let children = group.children().unwrap();

        let t1 = &children[0].children().unwrap()[0]
            .get_shape()
            .as_triangle()
            .unwrap();
        let t2 = &children[1].children().unwrap()[0]
            .get_shape()
            .as_triangle()
            .unwrap();

        assert_eq!(children.len(), 2);

        assert_eq!(t1.p1(), parser.get_vertex(1));
        assert_eq!(t1.p2(), parser.get_vertex(2));
        assert_eq!(t1.p3(), parser.get_vertex(3));

        assert_eq!(t2.p1(), parser.get_vertex(1));
        assert_eq!(t2.p2(), parser.get_vertex(3));
        assert_eq!(t2.p3(), parser.get_vertex(4));
    }
}
