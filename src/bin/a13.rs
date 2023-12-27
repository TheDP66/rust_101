fn main() {
    let numbers = vec![10, 20, 30, 40];
    for n in &numbers {
        match n {
            30 => println!("thirty"),
            _ => println!("{:?}", n),
        }
    }

    println!("length = {:?}", numbers.len())
}
