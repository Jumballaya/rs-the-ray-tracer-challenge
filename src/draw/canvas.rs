use std::{fmt::Display, fs::File, io::Write, path::Path};

use super::color::Color;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        f.write_str("255\n")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.to_index(&(x, y));
                let r = self.data.get(index).unwrap_or(&0);
                let g = self.data.get(index + 1).unwrap_or(&0);
                let b = self.data.get(index + 2).unwrap_or(&0);
                write!(f, "{} {} {} ", r, g, b)?;
                if x > 1 && x % 4 == 0 {
                    f.write_str("\n")?;
                }
            }
        }
        Ok(())
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height * 3],
        }
    }

    pub fn set_pixel(&mut self, pos: Position, color: &Color) {
        let (r, g, b) = color.scale();

        let index = self.to_index(&pos);
        if index + 2 < self.data.len() {
            self.data[index] = r;
            self.data[index + 1] = g;
            self.data[index + 2] = b;
        }
    }

    pub fn pixel_at(&self, pos: Position) -> Option<Color> {
        let index = self.to_index(&pos);
        if index + 2 < self.data.len() {
            let r: f64 = (self.data[index] as f64) / 255.0;
            let g: f64 = (self.data[index + 1] as f64) / 255.0;
            let b: f64 = (self.data[index + 2] as f64) / 255.0;
            return Some(Color::new(r, g, b));
        }
        None
    }

    pub fn save(&self, dir: &str, name: &str) -> std::io::Result<()> {
        let file_name = [dir, "/", name, ".ppm"].concat();
        let file_path = Path::new(file_name.as_str());
        let mut file = File::create(file_path)?;
        file.write_all(format!("{}", self).as_bytes())?;
        Ok(())
    }

    fn to_index(&self, (x, y): &Position) -> usize {
        ((y * self.width) + x) * 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for color_frag in c.data {
            assert_eq!(color_frag, 0);
        }
    }

    #[test]
    fn test_can_write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.set_pixel((2, 3), &red);
        let got = c.pixel_at((2, 3)).unwrap();
        assert_eq!(got, red);
    }

    #[test]
    fn test_can_construct_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut line_count = 0;
        for line in c.to_string().lines() {
            if line_count == 0 {
                assert_eq!(line, "P3")
            } else if line_count == 1 {
                assert_eq!(line, "5 3")
            } else if line_count == 2 {
                assert_eq!(line, "255")
            } else if line_count > 3 {
                break;
            }
            line_count += 1;
        }
    }

    #[test]
    fn test_can_construct_ppm_body() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.set_pixel((0, 0), &c1);
        c.set_pixel((2, 1), &c2);
        c.set_pixel((4, 2), &c3);

        let mut line_count = 0;
        for line in c.to_string().lines() {
            if line_count == 3 {
                assert_eq!(line, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ")
            } else if line_count == 4 {
                assert_eq!(line, "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 ")
            } else if line_count == 5 {
                assert_eq!(line, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ")
            } else if line_count > 6 {
                break;
            }
            line_count += 1;
        }
    }

    #[test]
    fn test_ppm_long_lines_are_split() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for y in 0..c.height {
            for x in 0..c.width {
                let pos = (x, y);
                c.set_pixel(pos, &color);
            }
        }

        let mut line_count = 0;
        for line in c.to_string().lines() {
            if line_count == 3 {
                assert_eq!(
                    line,
                    "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
                )
            } else if line_count == 4 {
                assert_eq!(line, "255 204 153 255 204 153 255 204 153 255 204 153 ")
            } else if line_count == 5 {
                assert_eq!(
                    line,
                    "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 "
                )
            } else if line_count == 6 {
                assert_eq!(line, "255 204 153 255 204 153 255 204 153 255 204 153 ")
            } else if line_count > 7 {
                break;
            }
            line_count += 1;
        }
    }
}
