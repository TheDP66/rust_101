struct Grocery {
    id: i32,
    qty: i32,
}

fn show_id(grocery: &Grocery) {
    println!("id: {:?}", grocery.id)
}

fn show_qty(grocery: &Grocery) {
    println!("qty: {:?}", grocery.qty)
}

fn main() {
    let grocery = Grocery { id: 1, qty: 20 };

    show_id(&grocery);
    show_qty(&grocery);
}
