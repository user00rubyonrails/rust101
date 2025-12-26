fn main() {
    let car = Car {
        category: String::from("vios"),
    };
    println!("car category: {}", car.get_category());
    println!("car speed: {}", car.speed());

    let bike = MotorBike {
        category: "Yamaha".to_owned(),
    };
    println!("bike category: {}", bike.get_category());
    println!("bike speed: {}", bike.speed());
}

// general methods for object
trait Vehicle {
    fn get_category(&self) -> &str;
    fn speed(&self) -> u8;
}

struct Car {
    category: String,
}

struct MotorBike {
    category: String,
}

// specified on the object `Car`
impl Vehicle for Car {
    fn get_category(&self) -> &str {
        &self.category
    }
    fn speed(&self) -> u8 {
        100
    }
}

// specified on the object `Vehicle`
impl Vehicle for MotorBike {
    fn get_category(&self) -> &str {
        &self.category
    }

    fn speed(&self) -> u8 {
        60
    }
}
