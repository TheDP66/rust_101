enum Ligth {
    Bright,
    Dull,
}

fn display_light(light: &Ligth) {
    match light {
        Ligth::Bright => println!("bright"),
        Ligth::Dull => println!("dull"),
    }
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages)
}

fn display_rating(book: &Book) {
    println!("pages = {:?}", book.rating)
}

fn main() {
    let dull = Ligth::Dull;
    display_light(&dull);
    display_light(&dull);

    let book = Book {
        pages: 5,
        rating: 9,
    };

    display_page_count(&book);
    display_rating(&book);
}
