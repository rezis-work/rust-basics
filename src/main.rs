fn main() {
    let red = Color::new(255, 0, 0);
    let purple = Color::new(255, 0, 255);
    let (r, g, b) = Color::rgb(purple);
    let (r2, g2, b2) = red.rgb();
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);
    println!("Red: {}, Green: {}, Blue: {}", r2, g2, b2);
}

enum Color {
    Green,
    Yellow,
    Red,
    Custom (u8, u8, u8)
}

impl Color {
    fn rgb(self) -> (u8, u8, u8) {
        return match self {
            Color::Green => (0, 255, 0),
            Color::Yellow => (255, 255, 0),
            Color::Red => (255, 0, 0),
            Color::Custom(r, g, b) => (r, g, b),
        };
    }
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color::Custom(r, g, b)
    }
}
