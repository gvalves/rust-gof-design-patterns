use command::{LightIntensityCommand, LightPowerCommand, SmartHouseApp, SmartHouseLight};

pub fn main() {
    let bedroom_light = SmartHouseLight::new("Bedroom light").into();
    let bathroom_light = SmartHouseLight::new("Bathroom light").into();

    let bedroom_light_power_command = LightPowerCommand::new(&bedroom_light).into();
    let bathroom_light_power_command = LightPowerCommand::new(&bathroom_light).into();
    let bedroom_light_intensity_command = LightIntensityCommand::new(&bedroom_light).into();

    let mut smart_house_app = SmartHouseApp::new();

    smart_house_app.add_command("btn-1", &bedroom_light_power_command);
    smart_house_app.add_command("btn-2", &bathroom_light_power_command);
    smart_house_app.add_command("btn-3", &bedroom_light_intensity_command);

    smart_house_app.execute_command("btn-1");
    smart_house_app.execute_command("btn-2");

    println!();

    for _ in 1..5 {
        smart_house_app.execute_command("btn-3");
    }

    for _ in 1..3 {
        smart_house_app.undo_command("btn-3");
    }
}
