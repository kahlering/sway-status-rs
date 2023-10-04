use std::io::Seek;
use crate::status_bar;

const POWER_SUPPLY_CAPACITY_PROPERTY: &str = "POWER_SUPPLY_CAPACITY=";
const POWER_SUPPLY_STATUS_PROPERTY: &str = "POWER_SUPPLY_STATUS=";
const POWER_SUPPLY_POWER_NOW_PROPERTY: &str = "POWER_SUPPLY_POWER_NOW=";


pub struct BatteryModule{
    f_uevent: std::fs::File,
    module_name: Option<String>,
    instance: Option<String>,
    last_update: std::time::Instant,
    refresh_rate_ms: u64,
}

impl status_bar::StatusModule for BatteryModule{
    
    // fn configure(&mut self, module_conf: &toml::Value) {
    //     let name = module_conf["name"].as_str();
    //     match name{
    //         None => {eprint!("could not read name from config file for battery module"); return;},
    //         Some(s) => {self.module_name = Some(String::from(s))}
    //     }
    // }

    fn get_instance_name(&self) -> Option<String> {
        self.instance.clone()
    }

    fn get_module_name(&self) -> Option<String> {
        self.module_name.clone()
    }

    fn handle_event(&mut self, _event: &status_bar::Event) {
    }

    fn get_update(&mut self) -> Option<status_bar::StatusUpdate>{
        if self.last_update.elapsed() < std::time::Duration::from_millis(self.refresh_rate_ms){
            return None;
        }
        self.last_update = std::time::Instant::now();

        self.f_uevent.seek(std::io::SeekFrom::Start(0)).expect("/sys/class/power_supply/BAT0/power_now");
        let uevent_string = std::io::read_to_string(&self.f_uevent).unwrap();

        let capacity: isize = Self::get_property_from_uevent_str(POWER_SUPPLY_CAPACITY_PROPERTY, uevent_string.as_str()).parse().unwrap();
        let status: &str = Self::get_property_from_uevent_str(POWER_SUPPLY_STATUS_PROPERTY, uevent_string.as_str());
        let power_now: f32 = Self::get_property_from_uevent_str(POWER_SUPPLY_POWER_NOW_PROPERTY, uevent_string.as_str()).parse().unwrap();

        Some(status_bar::StatusUpdate{
            full_text: String::from(format!("bat: {:.0}% {}{:.1}W", capacity, if status == "Charging\n" {'+'} else if status == "Discharging\n" {'-'} else {'?'}, power_now / 1000000.0)),
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
            name: self.module_name.clone(),
            instance: self.instance.clone(),
            separator: None,
            separator_block_width: None,
            markup: None
        })
    }

}



impl BatteryModule{
    pub fn from_config(module_conf: &toml::Value) -> Option<BatteryModule>{
        let name = module_conf.get("name")?.as_str()?;
        let refresh_rate_ms = module_conf.get("refresh_rate_ms").and_then(|v| {v.as_integer()}).or_else(||{eprintln!("battery module: could not read refresh_rate_ms from config"); None})?;
        let bat_uevent_path = module_conf.get("bat_uevent_path").and_then(|v|{v.as_str()}).or_else(||{eprintln!("battery module: could not read bat_uevent_path from config"); None})?;
        let file = std::fs::File::open(bat_uevent_path).ok().or_else(||{eprintln!("battery module: could not open file {}", bat_uevent_path); None})?;

        Some(BatteryModule{
            f_uevent: file,
            module_name: Some(String::from(name)),
            instance: None,
            last_update: std::time::Instant::now() - std::time::Duration::from_secs(refresh_rate_ms as u64),
            refresh_rate_ms: refresh_rate_ms as u64,
        })
    }

    fn get_property_from_uevent_str<'a>(property: &str, uevent_str: &'a str) -> &'a str{
        let idx1 = uevent_str.find(property).unwrap() + property.len();
        let idx2 = uevent_str[idx1..].find("\n").unwrap();
        &uevent_str[idx1..(idx1 +idx2)]
    }
}