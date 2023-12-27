struct Temperature {
    degrees_f: f64,
}

// ? implement show_temp to Temperature struct
impl Temperature {
    // ? change "temp: Temperature" to "&self"
    // fn show_temp(temp: Temperature) {
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f)
    }

    // return Static Temperature struct
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }
    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    // method 1
    Temperature::show_temp(&hot);
    // method 2
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
