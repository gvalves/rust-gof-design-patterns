use rand::{self, Rng};

use crate::{BycicleFactory, CarFactory, Vehicle, VehicleFactory};

pub fn random_number(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

pub fn random_vehicle() -> Box<dyn Vehicle> {
    let car1 = CarFactory::create_vehicle("Fusca");
    let car2 = CarFactory::create_vehicle("Celta Preto");
    let bycicle = BycicleFactory::create_vehicle("Bicleta");

    let mut vehicles = vec![car1, car2, bycicle];
    let max = vehicles.len();
    let random_index = random_number(max);

    return vehicles.remove(random_index);
}
