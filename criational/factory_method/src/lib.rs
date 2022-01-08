pub mod utils;

pub trait VehicleFactory {
    fn create_vehicle(name: &str) -> Box<dyn Vehicle>;
}

pub trait Vehicle {
    fn pick_up(&self, customer_name: &str);
    fn stop(&self);
}

pub struct Bycicle {
    name: String,
}

impl Bycicle {
    pub fn new(name: &str) -> Bycicle {
        let name = String::from(name);
        Bycicle { name }
    }
}

impl Vehicle for Bycicle {
    fn pick_up(&self, customer_name: &str) {
        println!("{} está buscando {}", self.name, customer_name);
    }

    fn stop(&self) {
        println!("{} parou", self.name);
    }
}

pub struct Car {
    name: String,
}

impl Car {
    pub fn new(name: &str) -> Car {
        let name = String::from(name);
        Car { name }
    }
}

impl Vehicle for Car {
    fn pick_up(&self, customer_name: &str) {
        println!("{} está buscando {}", self.name, customer_name);
    }

    fn stop(&self) {
        println!("{} parou", self.name);
    }
}

pub struct BycicleFactory;

impl VehicleFactory for BycicleFactory {
    fn create_vehicle(name: &str) -> Box<dyn Vehicle> {
        Box::new(Bycicle::new(name))
    }
}

pub struct CarFactory;

impl VehicleFactory for CarFactory {
    fn create_vehicle(name: &str) -> Box<dyn Vehicle> {
        Box::new(Car::new(name))
    }
}
