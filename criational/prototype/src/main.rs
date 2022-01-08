use prototype::{Address, Person};

fn main() {
    let address1 = Address::new("Av Brasil", 15);

    let mut person1 = Person::new("Gustavo", 20);
    person1.add_address(address1);

    let mut person2 = person1.clone();
    person2.set_name("Joana");

    person1.get_mut_addresses()[0].set_street("Bla Bla Bla");

    dbg!(person1);
    dbg!(person2);
}
