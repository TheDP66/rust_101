enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);
}
