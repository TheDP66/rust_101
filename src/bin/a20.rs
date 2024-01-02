use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);

    let mut total_stock = 0;
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;

        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };

        println!("item={:?}, stock={:?}", item, stock_count)
    }

    println!("total stock = {:?}", total_stock)
}
