fn display_message(variable: bool) {
    if variable {
        println!("hello")
    } else {
        println!("goodbye")
    }
}

fn main() {
    display_message(false)
}
