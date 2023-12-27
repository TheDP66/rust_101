#[warn(dead_code)]

// Enum from enumeration
enum Direction {
    // each option called Variant
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn arah_mana(go: Direction) -> &'static str {
    match go {
        Direction::Up => "atas",
        Direction::Down => "bawah",
        Direction::Left => "kiri",
        Direction::Right => "kanan",
    }
}

fn doko_desuka(go: Direction) -> &'static str {
    match go {
        Direction::Up => "ue",
        Direction::Down => "shita",
        Direction::Left => "hidari",
        Direction::Right => "migi",
    }
}

fn main() {
    let mut dir = which_way(Direction::Down);
    println!("{:?}", dir);

    let arah = arah_mana(Direction::Up);
    let doko = doko_desuka(Direction::Right);
    dir = arah_mana(Direction::Left);

    println!("{:?}", dir);
    println!("{:?}", arah);
    println!("{:?}", doko);
}
