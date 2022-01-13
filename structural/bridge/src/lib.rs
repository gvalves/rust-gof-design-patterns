pub trait Device {
    fn get_name(&self) -> &str;
    fn get_power(&self) -> bool;
    fn set_power(&mut self, power_status: bool);
    fn get_volume(&self) -> u32;
    fn set_volume(&mut self, volume: u32);
}

pub trait RemoteControlProtocol {
    fn toggle_power(&mut self);
}

pub trait RemoteControlWithVolumeProtocol
where
    Self: RemoteControlProtocol,
{
    fn volume_up(&mut self);
    fn volume_down(&mut self);
}

pub struct Radio {
    name: String,
    power: bool,
    volume: u32,
}

impl Radio {
    pub fn new() -> Radio {
        Radio {
            name: "Radio".to_string(),
            power: false,
            volume: 50,
        }
    }
}

impl Device for Radio {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_power(&self) -> bool {
        self.power
    }

    fn set_power(&mut self, power_status: bool) {
        self.power = power_status;
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }

    fn set_volume(&mut self, volume: u32) {
        if volume > 100 {
            return;
        }

        self.volume = volume;
    }
}

pub struct Tv {
    name: String,
    power: bool,
    volume: u32,
}

impl Tv {
    pub fn new() -> Tv {
        Tv {
            name: "Tv".to_string(),
            power: false,
            volume: 50,
        }
    }
}

impl Device for Tv {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_power(&self) -> bool {
        self.power
    }

    fn set_power(&mut self, power_status: bool) {
        self.power = power_status;
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }

    fn set_volume(&mut self, volume: u32) {
        if volume > 100 {
            return;
        }

        self.volume = volume;
    }
}

pub struct RemoteControl<T: Device> {
    device: T,
}

impl<T: Device> RemoteControl<T> {
    pub fn new(device: T) -> RemoteControl<T> {
        RemoteControl { device }
    }
}

impl<T: Device> RemoteControlProtocol for RemoteControl<T> {
    fn toggle_power(&mut self) {
        self.device.set_power(!self.device.get_power());

        println!(
            "{}, power status: {}",
            self.device.get_name(),
            self.device.get_power()
        );
    }
}

pub struct RemoteControlWithVolume<T: Device> {
    device: T,
}

impl<T: Device> RemoteControlWithVolume<T> {
    pub fn new(device: T) -> RemoteControlWithVolume<T> {
        RemoteControlWithVolume { device }
    }
}

impl<T: Device> RemoteControlProtocol for RemoteControlWithVolume<T> {
    fn toggle_power(&mut self) {
        self.device.set_power(!self.device.get_power());

        println!(
            "{}, power status: {}",
            self.device.get_name(),
            self.device.get_power()
        );
    }
}

impl<T: Device> RemoteControlWithVolumeProtocol for RemoteControlWithVolume<T> {
    fn volume_up(&mut self) {
        let old_volume = self.device.get_volume();

        self.device.set_volume(self.device.get_volume() + 10);

        println!(
            "The volume of {} was {} and now is {}",
            self.device.get_name(),
            old_volume,
            self.device.get_volume()
        )
    }

    fn volume_down(&mut self) {
        let old_volume = self.device.get_volume();

        self.device.set_volume(self.device.get_volume() - 10);

        println!(
            "The volume of {} was {} and now is {}",
            self.device.get_name(),
            old_volume,
            self.device.get_volume()
        )
    }
}

pub enum RemoteControlOption<T: Device> {
    Default(RemoteControl<T>),
    WithVolume(RemoteControlWithVolume<T>),
}
