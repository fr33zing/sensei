use std::process::Command;

pub trait Sensitivity {
    fn name() -> String;
    fn default_local() -> f64;
    fn local_to_normal(local: f64) -> f64;
    fn normal_to_local(normal: f64) -> f64;
    fn set(local: f64);

    fn set_normal(normal: f64) {
        Self::set(Self::normal_to_local(normal));
    }
    fn set_relative_normal(normal: f64) {
        let default_normal = Self::local_to_normal(Self::default_local());
        Self::set_normal(default_normal * normal);
    }
    fn reset() {
        Self::set_relative_normal(1f64);
    }
}

//
// Gnome
//

const GNOME_MOUSE_SPEED_PATH: &str = "/org/gnome/desktop/peripherals/mouse/speed";
const GNOME_MOUSE_SPEED_DEFAULT_VALUE: f64 = -0.95; // TODO get from a config file

pub struct GnomeSensitivity;

impl Sensitivity for GnomeSensitivity {
    fn name() -> String {
        "Gnome".into()
    }
    fn default_local() -> f64 {
        GNOME_MOUSE_SPEED_DEFAULT_VALUE
    }
    fn local_to_normal(local: f64) -> f64 {
        (local + 1f64) / 2f64
    }
    fn normal_to_local(normal: f64) -> f64 {
        normal * 2f64 - 1f64
    }
    fn set(local: f64) {
        Command::new("dconf")
            .arg("write")
            .arg(GNOME_MOUSE_SPEED_PATH)
            .arg(local.to_string())
            .output()
            .expect("dconf");
    }
}

//
// OpenRazer
//

const OPENRAZER_MOUSE_DEVICE: &str = "Razer DeathAdder V3 Pro (Wireless)";
const OPENRAZER_MOUSE_DPI_DEFAULT_VALUE: f64 = 30000.0;
const OPENRAZER_MOUSE_DPI_MAX: f64 = 30000.0;

pub struct OpenRazerSensitivity;

impl Sensitivity for OpenRazerSensitivity {
    fn name() -> String {
        "OpenRazer".into()
    }
    fn default_local() -> f64 {
        OPENRAZER_MOUSE_DPI_DEFAULT_VALUE
    }
    fn local_to_normal(local: f64) -> f64 {
        local / OPENRAZER_MOUSE_DPI_MAX
    }
    fn normal_to_local(normal: f64) -> f64 {
        normal * OPENRAZER_MOUSE_DPI_MAX
    }
    fn set(local: f64) {
        Command::new("razer-cli")
            .arg("--device")
            .arg(OPENRAZER_MOUSE_DEVICE)
            .arg("--dpi")
            .arg(local.to_string())
            .output()
            .expect("razer-cli");
    }
}
