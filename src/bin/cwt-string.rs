fn print_it(data: &str) {
    println!("{:?}", data);
}

struct Employee {
    name: String,
}

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    print_it("a string slice");

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");

    print_it(&owned_string);
    print_it(&another_owned);

    // method 1
    let emp_name = "Jayson".to_owned();
    // method 2
    let emp_name = String::from("Jayson");

    let emp = Employee { name: emp_name };

    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("name: {:?}, count: {:?}", item.name, item.count);
    }
}
