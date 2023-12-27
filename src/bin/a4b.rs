fn main() {
    let some_int = 6;

    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        // wildcard or default in switch
        _ => println!("its something else"),
    }
}
