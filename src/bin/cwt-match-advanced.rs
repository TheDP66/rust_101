enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        _ => println!("number: {:?}", n),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat discount of 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        // ? _ : wildcard
        // Discount::Flat(_) => println!("flat discount other"),
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket { price: 50, event } => println!("event @50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
