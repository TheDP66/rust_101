fn coordinate(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let (x, y) = coordinate(1, 1);

    if y > 5 {
        println!("> 5")
    } else if y < 5 {
        println!("< 5")
    } else {
        println!("= 5")
    }
}
