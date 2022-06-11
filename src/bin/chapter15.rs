use std::fs;

/**
 *
 * Triangles
 *
 */

fn main() {
    let contents = fs::read_to_string("./chapter12.ppm").expect("Error reading file chapter12.ppm");

    for line in contents.lines() {
        println!("{}", line);
    }
}
