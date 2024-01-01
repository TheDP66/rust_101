#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Age must be at least 21".to_string())
        }
    }
}

fn main() {
    let child = Adult::new(15, "Anita");
    let adult = Adult::new(99, "Sanjay");

    match child {
        Ok(c) => println!("{} is {} years old", c.name, c.age),
        Err(e) => println!("{e}"),
    }

    match adult {
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        Err(e) => println!("{e}"),
    }
}
