#[derive(Debug)]
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("blue"),
            age: 28,
        },
        Person {
            name: "Anna".to_owned(),
            fav_color: "red".to_owned(),
            age: 19,
        },
        Person {
            name: "Kattie".to_string(),
            fav_color: "green".to_string(),
            age: 23,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
