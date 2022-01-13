use bridge::{
    Device, Radio, RemoteControl, RemoteControlOption, RemoteControlProtocol,
    RemoteControlWithVolume, RemoteControlWithVolumeProtocol, Tv,
};

pub fn main() {
    let radio = Radio::new();
    let mut radio_remote_control = RemoteControl::new(radio);

    let radio = Radio::new();
    let mut radio_remote_control_with_volume = RemoteControlWithVolume::new(radio);

    let tv = Tv::new();
    let mut tv_remote_control = RemoteControl::new(tv);

    let tv = Tv::new();
    let mut tv_remote_control_with_volume = RemoteControlWithVolume::new(tv);

    client_code(&mut tv_remote_control);
    client_code(&mut tv_remote_control_with_volume);
    client_code(&mut radio_remote_control);
    client_code(&mut radio_remote_control_with_volume);

    client_code_enum(&mut RemoteControlOption::Default(tv_remote_control));
    client_code_enum(&mut RemoteControlOption::WithVolume(
        tv_remote_control_with_volume,
    ));
    client_code_enum(&mut RemoteControlOption::Default(radio_remote_control));
    client_code_enum(&mut RemoteControlOption::WithVolume(
        radio_remote_control_with_volume,
    ));
}

pub fn client_code<T: RemoteControlProtocol>(remote_control: &mut T) {
    remote_control.toggle_power();
}

pub fn client_code_enum<T: Device>(remote_control: &mut RemoteControlOption<T>) {
    match remote_control {
        RemoteControlOption::Default(remote) => remote.toggle_power(),
        RemoteControlOption::WithVolume(remote) => {
            remote.toggle_power();
            remote.volume_up();
            remote.volume_up();
            remote.volume_up();
            remote.volume_down();
        }
    }
}
