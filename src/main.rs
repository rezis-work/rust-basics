fn main() {
    let current_color = Color::Yellow;

    match current_color {
        Color::Green => {
            println!("It was green!")
        }
        Color::Yellow => {
            println!("It was yellow!")
        }
        Color::Red => {
            println!("It was red!")
        }
        Color::Custom(r, g, b) => {
            println!("It was custom! {} {} {}", r, g, b)
        }
    }


}

enum Color {
    Green,
    Yellow,
    Red,
    Custom (u8, u8, u8)
}
