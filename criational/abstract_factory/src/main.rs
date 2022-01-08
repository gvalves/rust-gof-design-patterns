use abstract_factory::{
    CustomerVehicleFactory, EnterpriseCustomerVehicleFactory, IndividualCustomerVehicleFactory,
};

fn main() {
    let individual_car = IndividualCustomerVehicleFactory::create_vehicle("Fusca");
    let individual_customer = IndividualCustomerVehicleFactory::create_customer("João");

    let enterprise_car = EnterpriseCustomerVehicleFactory::create_vehicle("Celta");
    let enterprise_customer = EnterpriseCustomerVehicleFactory::create_customer("Helena");

    individual_car.pick_up(individual_customer);
    enterprise_car.pick_up(enterprise_customer);
}
