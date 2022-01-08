pub trait Vehicle {
    fn pick_up(&self, customer: Box<dyn Customer>);
    fn get_name(&self) -> &str;
}

pub trait Customer {
    fn get_name(&self) -> &str;
}

pub trait CustomerVehicleFactory {
    fn create_customer(name: &str) -> Box<dyn Customer>;
    fn create_vehicle(name: &str) -> Box<dyn Vehicle>;
}

pub struct IndividualCar {
    name: String,
}

impl IndividualCar {
    pub fn new(name: &str) -> IndividualCar {
        let name = String::from(name);

        IndividualCar { name }
    }
}

impl Vehicle for IndividualCar {
    fn pick_up(&self, customer: Box<dyn Customer>) {
        println!(
            "{} está buscando {} (INDIVIDUAL)",
            self.name,
            customer.get_name()
        );
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct EnterpriseCar {
    name: String,
}

impl EnterpriseCar {
    pub fn new(name: &str) -> EnterpriseCar {
        let name = String::from(name);

        EnterpriseCar { name }
    }
}

impl Vehicle for EnterpriseCar {
    fn pick_up(&self, customer: Box<dyn Customer>) {
        println!(
            "{} está buscando {} (ENTERPRISE)",
            self.name,
            customer.get_name()
        );
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct IndividualCustomer {
    name: String,
}

impl IndividualCustomer {
    pub fn new(name: &str) -> IndividualCustomer {
        let name = String::from(name);

        IndividualCustomer { name }
    }
}

impl Customer for IndividualCustomer {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct EnterpriseCustomer {
    name: String,
}

impl EnterpriseCustomer {
    pub fn new(name: &str) -> EnterpriseCustomer {
        let name = String::from(name);

        EnterpriseCustomer { name }
    }
}

impl Customer for EnterpriseCustomer {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct IndividualCustomerVehicleFactory;

impl CustomerVehicleFactory for IndividualCustomerVehicleFactory {
    fn create_customer(name: &str) -> Box<dyn Customer> {
        Box::new(IndividualCustomer::new(name))
    }

    fn create_vehicle(name: &str) -> Box<dyn Vehicle> {
        Box::new(IndividualCar::new(name))
    }
}

pub struct EnterpriseCustomerVehicleFactory;

impl CustomerVehicleFactory for EnterpriseCustomerVehicleFactory {
    fn create_customer(name: &str) -> Box<dyn Customer> {
        Box::new(EnterpriseCustomer::new(name))
    }

    fn create_vehicle(name: &str) -> Box<dyn Vehicle> {
        Box::new(EnterpriseCar::new(name))
    }
}
