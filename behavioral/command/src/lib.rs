use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait SmartHouseCommand {
    fn execute(&self);
    fn undo(&self);
}

pub struct SmartHouseLight {
    on: bool,
    intensity: u32,
    name: String,
}

impl SmartHouseLight {
    pub fn new(name: &str) -> Self {
        let name = String::from(name);

        Self {
            on: false,
            intensity: 50,
            name,
        }
    }

    pub fn get_power_status(&self) -> &str {
        if self.is_on() {
            "ON"
        } else {
            "OFF"
        }
    }

    pub fn on(&mut self) {
        if self.is_on() {
            return;
        }

        self.on = true;
        println!("{} is {}", self.name(), self.get_power_status());
    }

    pub fn off(&mut self) {
        if !self.is_on() {
            return;
        }

        self.on = false;
        println!("{} is {}", self.name(), self.get_power_status());
    }

    pub fn increase_intensity(&mut self) {
        if self.intensity() >= 100 {
            return;
        }

        self.intensity += 5;
    }

    pub fn decrease_intensity(&mut self) {
        if self.intensity() <= 0 {
            return;
        }

        self.intensity -= 5;
    }

    /// Get a reference to the smart house light's on.
    pub fn is_on(&self) -> bool {
        self.on
    }

    /// Get a reference to the smart house light's intensity.
    pub fn intensity(&self) -> u32 {
        self.intensity
    }

    /// Get a reference to the smart house light's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl From<SmartHouseLight> for Rc<RefCell<SmartHouseLight>> {
    fn from(light: SmartHouseLight) -> Self {
        Rc::new(RefCell::new(light))
    }
}

pub struct LightPowerCommand {
    light: Rc<RefCell<SmartHouseLight>>,
}

impl LightPowerCommand {
    pub fn new(light: &Rc<RefCell<SmartHouseLight>>) -> Self {
        let light = Rc::clone(light);
        Self { light }
    }
}

impl SmartHouseCommand for LightPowerCommand {
    fn execute(&self) {
        self.light.borrow_mut().on();
    }

    fn undo(&self) {
        self.light.borrow_mut().off();
    }
}

impl From<LightPowerCommand> for Rc<RefCell<dyn SmartHouseCommand>> {
    fn from(value: LightPowerCommand) -> Self {
        Rc::new(RefCell::new(value))
    }
}

pub struct LightIntensityCommand {
    light: Rc<RefCell<SmartHouseLight>>,
}

impl LightIntensityCommand {
    pub fn new(light: &Rc<RefCell<SmartHouseLight>>) -> Self {
        let light = Rc::clone(light);
        Self { light }
    }
}

impl SmartHouseCommand for LightIntensityCommand {
    fn execute(&self) {
        self.light.borrow_mut().increase_intensity();

        println!(
            "The intensity of {} is {}",
            self.light.borrow().name(),
            self.light.borrow().intensity()
        );
    }

    fn undo(&self) {
        self.light.borrow_mut().decrease_intensity();

        println!(
            "The intensity of {} is {}",
            self.light.borrow().name(),
            self.light.borrow().intensity()
        );
    }
}

impl From<LightIntensityCommand> for Rc<RefCell<dyn SmartHouseCommand>> {
    fn from(value: LightIntensityCommand) -> Self {
        Rc::new(RefCell::new(value))
    }
}

pub struct SmartHouseApp {
    commands: HashMap<String, Rc<RefCell<dyn SmartHouseCommand>>>,
}

impl SmartHouseApp {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn add_command(&mut self, key: &str, command: &Rc<RefCell<dyn SmartHouseCommand>>) {
        let command = Rc::clone(command);
        if !self.commands.contains_key(key) {
            self.commands.insert(key.to_string(), command);
        }
    }

    pub fn execute_command(&self, key: &str) {
        if let Some(command) = self.commands.get(key) {
            command.borrow().execute();
        }
    }

    pub fn undo_command(&self, key: &str) {
        if let Some(command) = self.commands.get(key) {
            command.borrow().undo();
        }
    }
}
