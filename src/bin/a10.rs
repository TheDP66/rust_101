fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let n = 11;

    let message = if n > 100 { "its big" } else { "its small" };

    println!("{:?}", message);

    print_message(n > 100)
}
