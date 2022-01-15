pub trait DeliveryFlyweight {
    fn deliver(&self, extrinsic_state: DeliveryLocationExtrinsicState);
}

pub struct DeliveryLocationExtrinsicState {
    client_name: String,
    location_number: String,
}

impl DeliveryLocationExtrinsicState {
    pub fn new(client_name: &str, location_number: &str) -> Self {
        let client_name = String::from(client_name);
        let location_number = String::from(location_number);

        Self {
            client_name,
            location_number,
        }
    }

    /// Get a reference to the delivery location extrinsic state's client.
    pub fn client_name(&self) -> &str {
        self.client_name.as_ref()
    }

    /// Get a reference to the delivery location extrinsic state's location number.
    pub fn location_number(&self) -> &str {
        self.location_number.as_ref()
    }
}

#[derive(Debug)]
pub struct DeliveryLocation {
    intrinsic_state: DeliveryLocationIntrinsicState,
}

impl DeliveryLocation {
    pub fn new(intrinsic_state: DeliveryLocationIntrinsicState) -> Self {
        Self { intrinsic_state }
    }

    pub fn street(&self) -> &str {
        self.intrinsic_state.street()
    }

    pub fn city(&self) -> &str {
        self.intrinsic_state.city()
    }
}

impl DeliveryFlyweight for DeliveryLocation {
    fn deliver(&self, extrinsic_state: DeliveryLocationExtrinsicState) {
        println!("Entrega para {}", extrinsic_state.client_name());
        println!("Em {} {}", self.street(), self.city());
        println!("Número: {}", extrinsic_state.location_number());
        println!("{}", String::from("-").repeat(50));
    }
}

#[derive(Debug)]
pub struct DeliveryLocationIntrinsicState {
    street: String,
    city: String,
}

impl DeliveryLocationIntrinsicState {
    pub fn new(street: &str, city: &str) -> Self {
        let street = String::from(street);
        let city = String::from(city);

        Self { street, city }
    }

    /// Get a reference to the delivery location intrinsic state's street.
    pub fn street(&self) -> &str {
        self.street.as_ref()
    }

    /// Get a reference to the delivery location intrinsic state's city.
    pub fn city(&self) -> &str {
        self.city.as_ref()
    }
}

#[derive(Debug)]
pub struct DeliveryLocationDictionary {
    ids: Vec<String>,
    locations: Vec<DeliveryLocation>,
}

impl DeliveryLocationDictionary {
    pub fn new() -> Self {
        Self {
            ids: vec![],
            locations: vec![],
        }
    }

    pub fn insert(&mut self, id: String, location: DeliveryLocation) -> &DeliveryLocation {
        let index = self.ids.iter().position(|_id| &id == _id);
        let location_ref;

        match index {
            Some(index) => location_ref = &self.locations[index],
            None => {
                self.ids.push(id);
                self.locations.push(location);
                location_ref = self.locations.last().unwrap();
            }
        }

        location_ref
    }
}

#[derive(Debug)]
pub struct DeliveryFactory {
    locations: DeliveryLocationDictionary,
}

impl DeliveryFactory {
    pub fn new() -> Self {
        Self {
            locations: DeliveryLocationDictionary::new(),
        }
    }

    pub fn make_location(
        &mut self,
        intrinsic_state: DeliveryLocationIntrinsicState,
    ) -> &DeliveryLocation {
        let id = DeliveryFactory::create_id(&intrinsic_state);
        let location = DeliveryLocation::new(intrinsic_state);

        self.locations.insert(id, location)
    }

    pub fn create_id(intrinsic_state: &DeliveryLocationIntrinsicState) -> String {
        let mut id = String::new();

        id.push_str(intrinsic_state.street());
        id.push_str("_");
        id.push_str(intrinsic_state.city());

        id = id.replace(" ", "").to_lowercase();

        id
    }

    /// Get a reference to the delivery factory's locations.
    pub fn locations(&self) -> &DeliveryLocationDictionary {
        &self.locations
    }
}

pub fn delivery_context(
    factory: &mut DeliveryFactory,
    client_name: &str,
    location_number: &str,
    street: &str,
    city: &str,
) {
    let extrinsic_state = DeliveryLocationExtrinsicState::new(client_name, location_number);
    let intrinsic_state = DeliveryLocationIntrinsicState::new(street, city);
    let location = factory.make_location(intrinsic_state);

    location.deliver(extrinsic_state);
}
