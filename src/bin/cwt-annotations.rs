enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn print_many(msg: &str, count: i32) {}

fn main() {
    let num: i32 = 15;
    let a: char = 'a';
    let left_cilck: Mouse = Mouse::LeftClick;

    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b'];
    let clicks: Vec<Mouse> = vec![Mouse::LeftClick, Mouse::LeftClick, Mouse::RightClick];
}
