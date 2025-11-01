fn main() {
    let point = new_point(1, 2, 3);
    println!("x: {}, y: {}, z: {}", point.x, point.y, point.z);
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn new_point(x: i64, y: i64, z: i64) -> Point {
   Point { x, y, z }
}

