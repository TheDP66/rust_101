#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Haha".to_owned(),
        locker: None,
    };

    let student2 = Student {
        name: "Hihi".to_owned(),
        locker: Some(13),
    };

    match student2.locker {
        Some(num) => println!("{:?} locker number is {:?}", student2.name, num),
        None => println!("{:?} dont have locker", student.name),
    }

    println!("{:?}", student);
    println!("{:?}", student2);
}
