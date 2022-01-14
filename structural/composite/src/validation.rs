use std::{any::Any, rc::Rc};

use regex::Regex;

type Validation = Rc<dyn ValidationComponent>;

pub trait ValidationComponent {
    fn get_name(&self) -> &str {
        ""
    }

    fn validate(&self, value: Rc<dyn Any>) -> bool;
}

pub struct ValidationEmail {
    name: String,
}

impl ValidationEmail {
    pub fn new() -> ValidationEmail {
        ValidationEmail {
            name: "ValidationEmail".to_string(),
        }
    }
}

impl ValidationComponent for ValidationEmail {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self, value: Rc<dyn Any>) -> bool {
        let text = match value.downcast_ref::<String>() {
            Some(text) => text,
            None => return false,
        };

        let re = match Regex::new("@") {
            Ok(re) => re,
            Err(_) => return false,
        };

        re.is_match(text)
    }
}

impl From<ValidationEmail> for Validation {
    fn from(validation: ValidationEmail) -> Self {
        Rc::new(validation)
    }
}

pub struct ValidationString {
    name: String,
}

impl ValidationString {
    pub fn new() -> ValidationString {
        ValidationString {
            name: "ValidationString".to_string(),
        }
    }
}

impl ValidationComponent for ValidationString {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self, value: Rc<dyn Any>) -> bool {
        match value.downcast_ref::<String>() {
            Some(_) => true,
            None => false,
        }
    }
}

impl From<ValidationString> for Validation {
    fn from(validation: ValidationString) -> Self {
        Rc::new(validation)
    }
}

pub struct ValidationNumber {
    name: String,
}

impl ValidationNumber {
    pub fn new() -> ValidationNumber {
        ValidationNumber {
            name: "ValidationNumber".to_string(),
        }
    }
}

impl ValidationComponent for ValidationNumber {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self, value: Rc<dyn Any>) -> bool {
        let text = match value.downcast_ref::<String>() {
            Some(text) => text,
            None => return false,
        };

        let re = match Regex::new(r"\d+") {
            Ok(re) => re,
            Err(_) => return false,
        };

        re.is_match(text)
    }
}

impl From<ValidationNumber> for Validation {
    fn from(validation: ValidationNumber) -> Self {
        Rc::new(validation)
    }
}

pub struct ValidationComposite {
    children: Vec<Validation>,
}

impl ValidationComposite {
    pub fn new() -> ValidationComposite {
        ValidationComposite { children: vec![] }
    }

    pub fn add(&mut self, validations: &mut Vec<Validation>) {
        // self.children.append(validations)
    }

    pub fn remove(&mut self, validation: Validation) {
        let position = self
            .children
            .iter()
            .position(|child| validation.get_name() == child.get_name());

        let index = match position {
            Some(index) => index,
            None => return,
        };

        self.children.remove(index);
    }
}

impl ValidationComponent for ValidationComposite {
    fn validate(&self, value: Rc<dyn Any>) -> bool {
        self.children
            .iter()
            .all(|validation| validation.validate(Rc::clone(&value)))
    }
}

impl From<ValidationComposite> for Validation {
    fn from(validation: ValidationComposite) -> Self {
        Rc::new(validation)
    }
}
