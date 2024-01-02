use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Cathy", 22);
    people.remove("Susan");

    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age),
        None => println!("not found"),
    }

    // ? iterate all data stored in hashmap
    for (person, age) in people.iter() {
        println!("person = {:?}, age = {:?}", person, age);
    }

    // ? iterate all keys stored in hashmap
    for person in people.keys() {
        println!("person = {:?}", person);
    }

    // ? iterate all values stored in hashmap
    for age in people.values() {
        println!("age = {:?}", age)
    }

    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "shirt".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "gym shorts".to_owned(),
        },
    );

    for (locker_number, content) in lockers.iter() {
        println!("number: {:?} content: {:?}", locker_number, content)
    }
}
