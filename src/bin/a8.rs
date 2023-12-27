enum Flavor {
    Coffee,
    Chocolate,
    Matcha,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn get_drink_detail(drink: Drink) {
    match drink.flavor {
        Flavor::Coffee => println!("coffee"),
        Flavor::Chocolate => println!("chocolate"),
        Flavor::Matcha => println!("matcha"),
    }

    println!("oz: {:?}", drink.ounces)
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Chocolate,
        ounces: 3.5,
    };

    get_drink_detail(my_drink);
}
