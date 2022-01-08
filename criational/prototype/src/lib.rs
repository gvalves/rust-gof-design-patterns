#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age: usize,
    addresses: Vec<Address>,
}

impl Person {
    pub fn new(name: &str, age: usize) -> Person {
        let name = String::from(name);

        Person {
            name,
            age,
            addresses: vec![],
        }
    }

    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn remove_address(&mut self, index: usize) {
        self.addresses.remove(index);
    }

    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    pub fn set_name(&mut self, name: &str) {
        let name = String::from(name);
        self.name = name;
    }

    pub fn get_age(&self) -> usize {
        self.age
    }

    pub fn set_age(&mut self, age: usize) {
        self.age = age;
    }

    pub fn get_addresses(&self) -> &[Address] {
        &self.addresses[..]
    }

    pub fn get_mut_addresses(&mut self) -> &mut [Address] {
        &mut self.addresses[..]
    }
}

#[derive(Clone, Debug)]
pub struct Address {
    street: String,
    number: usize,
}

impl Address {
    pub fn new(street: &str, number: usize) -> Address {
        let street = String::from(street);

        Address { street, number }
    }

    pub fn get_street(&self) -> &str {
        &self.street[..]
    }

    pub fn set_street(&mut self, street: &str) {
        let street = String::from(street);
        self.street = street;
    }

    pub fn get_number(&self) -> usize {
        self.number
    }

    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }
}
