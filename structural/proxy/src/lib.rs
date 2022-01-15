use std::rc::Rc;

use async_trait::async_trait;
use utils::async_sleep;

mod utils;

#[async_trait]
pub trait SystemUserProtocol {
    async fn get_addresses(&self) -> Vec<Address>;
}

#[derive(Debug)]
pub struct Address {
    street: String,
    number: usize,
}

impl Address {
    pub fn new(street: &str, number: usize) -> Self {
        let street = String::from(street);
        Self { street, number }
    }

    /// Get a reference to the address's street.
    pub fn street(&self) -> &str {
        self.street.as_ref()
    }

    /// Get a reference to the address's number.
    pub fn number(&self) -> usize {
        self.number
    }
}

#[derive(Debug)]
pub struct AdminUser {
    first_name: String,
    username: String,
}

impl AdminUser {
    pub fn new(first_name: &str, username: &str) -> Self {
        let first_name = String::from(first_name);
        let username = String::from(username);

        Self {
            first_name,
            username,
        }
    }

    /// Get a reference to the admin user's first name.
    pub fn first_name(&self) -> &str {
        self.first_name.as_ref()
    }

    /// Get a reference to the admin user's username.
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }
}

#[async_trait]
impl SystemUserProtocol for AdminUser {
    async fn get_addresses(&self) -> Vec<Address> {
        async_sleep(2000).await;

        vec![Address::new("Av Brasil", 50), Address::new("Rua A", 40)]
    }
}

impl From<AdminUser> for Rc<dyn SystemUserProtocol> {
    fn from(user: AdminUser) -> Self {
        Rc::new(user)
    }
}

pub struct SystemUserProxy {
    first_name: String,
    username: String,
    user: Option<Rc<dyn SystemUserProtocol>>,
    user_addresses: Option<Vec<Address>>,
}

impl SystemUserProxy {
    pub fn new(first_name: &str, username: &str) -> Self {
        let first_name = String::from(first_name);
        let username = String::from(username);

        Self {
            first_name,
            username,
            user: None,
            user_addresses: None,
        }
    }

    /// Get a reference to the system user proxy's user.
    pub fn user(&mut self) -> Rc<dyn SystemUserProtocol> {
        match self.user.as_ref() {
            Some(user) => Rc::clone(user),
            None => AdminUser::new(&self.first_name, &self.username).into(),
        }
    }

    pub async fn get_addresses(&mut self) -> &[Address] {
        let user = self.user();
        let addresses = self.user_addresses.take();

        match addresses {
            Some(addresses) => {
                self.user_addresses = Some(addresses);
            }
            None => {
                let addresses = user.get_addresses().await;
                self.user_addresses = Some(addresses);
            }
        }

        return self.user_addresses.as_ref().unwrap();
    }
}
