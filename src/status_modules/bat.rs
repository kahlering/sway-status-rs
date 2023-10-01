use std::io::Seek;
use serde_json::from_str;
use crate::status_bar;

const MODULE_NAME: &str = "battery_module";
const INSTANCE_NAME: &str = "battery_instance_1";

pub struct BatteryModule{
    f_energy_now: std::fs::File,
    f_energy_full: std::fs::File,
    f_status: std::fs::File,
    f_power_now: std::fs::File,
    
}

impl status_bar::StatusModule for BatteryModule{
    fn get_instance_name(&self) -> Option<String> {
        Some(String::from(INSTANCE_NAME))
    }

    fn get_module_name(&self) -> Option<String> {
        Some(String::from(MODULE_NAME))
    }

    fn handle_event(&self, _event: &status_bar::Event) {
    }

    fn get_status_block(&mut self) -> status_bar::StatusBlock{
        self.f_energy_now.seek(std::io::SeekFrom::Start(0)).expect("failed to seek in file /sys/class/power_supply/BAT0/energy_now");
        let energy_now: f32 = from_str(std::io::read_to_string(&self.f_energy_now).unwrap().as_str()).unwrap(); //todo err handling

        self.f_energy_full.seek(std::io::SeekFrom::Start(0)).expect("failed to seek in file /sys/class/power_supply/BAT0/energy_full");
        let energy_full: f32 = from_str(std::io::read_to_string(&self.f_energy_full).unwrap().as_str()).unwrap(); //todo err handling

        self.f_status.seek(std::io::SeekFrom::Start(0)).expect("failed to seek in file /sys/class/power_supply/BAT0/status");
        let status = std::io::read_to_string(&self.f_status).unwrap(); //todo err handling

        self.f_power_now.seek(std::io::SeekFrom::Start(0)).expect("/sys/class/power_supply/BAT0/power_now");
        let power_now: f32 = from_str(std::io::read_to_string(&self.f_power_now).unwrap().as_str()).unwrap(); //todo err handling
       
        status_bar::StatusBlock{
            full_text: String::from(format!("bat: {:.0}% {}{:.1}W", (energy_now / energy_full) * 100.0, if status == "Charging\n" {'+'} else if status == "Discharging\n" {'-'} else {'?'}, power_now / 1000000.0)),
            short_text: None,
            color: None,
            background: None,
            border: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
            min_width: None,
            align: None,
            urgent: None,
            name: Some(String::from(MODULE_NAME)),
            instance: Some(String::from(INSTANCE_NAME)),
            separator: None,
            separator_block_width: None,
            markup: None
        }
    }

    
}



impl BatteryModule{
    pub fn new() -> Result<BatteryModule, std::io::Error>{
        Ok(BatteryModule{
            f_energy_now: std::fs::File::open("/sys/class/power_supply/BAT0/energy_now")?,
            f_energy_full: std::fs::File::open("/sys/class/power_supply/BAT0/energy_full")?,
            f_status: std::fs::File::open("/sys/class/power_supply/BAT0/status")?,
            f_power_now: std::fs::File::open("/sys/class/power_supply/BAT0/power_now")?,
        })
    }
}