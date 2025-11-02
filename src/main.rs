fn main() {
    let red = Color::new(255, 0, 0);
    let purple = Color::new(255, 0, 255);
    let (r, g, b) = Color::rgb(purple);
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);

}

enum Color {
    Green,
    Yellow,
    Red,
    Custom (u8, u8, u8)
}

impl Color {
    fn rgb(color: Color) -> (u8, u8, u8) {
        return match color {
            Color::Green => (0, 255, 0),
            Color::Yellow => (255, 255, 0),
            Color::Red => (255, 0, 0),
            Color::Custom(r, g, b) => (r, g, b),
        };
    }
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color::Custom(r, g, b)
    }
}
