#[derive(Debug)]
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Colors,
}

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Blue => println!("blue"),
            Colors::Red => println!("red"),
            Colors::Green => println!("green"),
        }
    }
}

impl ShippingBox {
    fn print(&self) {
        println!("{:?}", self);
        self.color.print()
    }

    fn new(weight: f64, color: Colors, dimensions: Dimensions) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }
}

fn main() {
    let my_box = ShippingBox {
        color: Colors::Green,
        dimensions: Dimensions {
            depth: 30.2,
            height: 69.2,
            width: 40.0,
        },
        weight: 33.87,
    };
    my_box.print();

    let new_box = ShippingBox::new(
        234.0,
        Colors::Red,
        Dimensions {
            width: 224.32,
            height: 147.4,
            depth: 30.07,
        },
    );
    new_box.print();
}
