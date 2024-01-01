// ? Option<T> : optional field, cant be filled ( Some(n) ) or blank ( None )

struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {
            name: "bananas".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "eggs".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "bread".to_owned(),
            qty: 4,
        },
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }

    None
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "m@ex.com".to_owned(),
    };

    let becky = Customer {
        age: None,
        email: "b@ex.com".to_owned(),
    };

    match mark.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }

    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("qi: {:?}", ans),
        None => println!("q1: no response"),
    }
}
