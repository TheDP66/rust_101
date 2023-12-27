struct Test {
    score: i32,
}

fn main() {
    let my_number = vec![1, 2, 3];
    println!("{:?}", my_number);

    let mut my_number = Vec::new();
    my_number.push(1);
    my_number.push(2);
    my_number.push(3);
    my_number.pop();
    my_number.len();
    println!("{:?}", my_number);

    let two = my_number[1];
    println!("{two}");

    for num in my_number {
        println!("{:?}", num)
    }

    let my_scores = vec![
        Test { score: 90 },
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 93 },
    ];

    for test in my_scores {
        println!("score = {:?}", test.score)
    }
}
