use std::fs;

use json;
use regex::Regex;

pub trait CustomerDataParser {
    fn fix_customer_data(&self) -> Vec<CustomerData> {
        let customers_data = self.parse_data();
        self.after_parse_data();
        self.fix_cpf(customers_data)
    }

    fn parse_data(&self) -> Vec<CustomerData>;

    fn after_parse_data(&self) {}

    fn fix_cpf(&self, customers_data: Vec<CustomerData>) -> Vec<CustomerData> {
        let re = Regex::new(r"\D").unwrap();

        customers_data
            .into_iter()
            .map(|customer| {
                let mut customer = customer;
                let cpf = customer.cpf();
                let fixed_cpf = re.replace(cpf, "");
                let fixed_cpf = fixed_cpf.to_string();

                customer.set_cpf(fixed_cpf);
                customer
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct CustomerData {
    name: String,
    age: u32,
    cpf: String,
}

impl CustomerData {
    pub fn new(name: &str, age: u32, cpf: &str) -> Self {
        let name = name.to_string();
        let cpf = cpf.to_string();
        Self { name, age, cpf }
    }

    /// Get a reference to the customer data's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the customer data's age.
    pub fn age(&self) -> u32 {
        self.age
    }

    /// Get a reference to the customer data's cpf.
    pub fn cpf(&self) -> &str {
        self.cpf.as_ref()
    }

    /// Set the customer data's cpf.
    pub fn set_cpf(&mut self, cpf: String) {
        self.cpf = cpf;
    }
}

pub struct CustomerDataParserTxt {
    file_path: String,
}

impl CustomerDataParserTxt {
    pub fn new(file_path: &str) -> Self {
        let file_path = file_path.to_string();
        Self { file_path }
    }
}

impl CustomerDataParser for CustomerDataParserTxt {
    fn parse_data(&self) -> Vec<CustomerData> {
        let path = &self.file_path;
        let data = fs::read_to_string(path).unwrap();
        let lines = data.split_terminator("\n");
        let mut customers_data = vec![];

        lines.for_each(|line| {
            let line_data: Vec<&str> = line.split("\t").collect();
            let name = line_data[0];
            let age = match line_data[1].parse() {
                Ok(age) => age,
                Err(_) => return,
            };
            let cpf = line_data[2];

            customers_data.push(CustomerData::new(name, age, cpf));
        });

        customers_data
    }
}

pub struct CustomerDataParserJson {
    file_path: String,
}

impl CustomerDataParserJson {
    pub fn new(file_path: &str) -> Self {
        let file_path = file_path.to_string();
        Self { file_path }
    }
}

impl CustomerDataParser for CustomerDataParserJson {
    fn parse_data(&self) -> Vec<CustomerData> {
        let path = &self.file_path;
        let data = fs::read_to_string(path).unwrap();
        let mut json_data = json::parse(&data).unwrap();
        let mut customers_data = vec![];

        loop {
            let customer_data = json_data.pop();

            if customer_data.is_null() {
                break;
            }

            let name = match customer_data["name"].as_str() {
                Some(name) => name,
                None => continue,
            };
            let age = match customer_data["age"].as_str() {
                Some(age) => match age.parse() {
                    Ok(age) => age,
                    Err(_) => continue,
                },
                None => continue,
            };
            let cpf = match customer_data["cpf"].as_str() {
                Some(cpf) => cpf,
                None => continue,
            };

            let customer_data = CustomerData::new(name, age, cpf);

            customers_data.push(customer_data);
        }

        customers_data
    }

    fn after_parse_data(&self) {
        println!("Hook after_parse_data was called");
    }
}
