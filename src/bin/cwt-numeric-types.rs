fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let mult = 5 * 5;

    let five = sub(8, 3);

    let rem = 6 % 3;

    println!("sum: {sum}");
    println!("value: {value}");
    println!("division: {division}");
    println!("mult: {mult}");
    println!("five: {five}");
    println!("rem: {rem}");
}
