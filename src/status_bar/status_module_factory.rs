use crate::status_bar::StatusModule;
use crate::status_modules;

pub fn create_status_module(module_type: &str, module_conf: &toml::Value) -> Option<Box<dyn StatusModule>>{
    let m: Box<dyn StatusModule> = match module_type{
        "battery" => {Box::new(status_modules::BatteryModule::from_config(module_conf).ok()?)},
        "date_and_time" => {Box::new(status_modules::DateAndTimeModule::from_config(module_conf).ok()?)},
        "audio_volume" => {Box::new(status_modules::AudioModule::from_config(module_conf).ok()?)},
        "sysinfo" => {Box::new(status_modules::SysInfoModule::from_config(module_conf).ok()?)},
        _ => return None,
    };
    Some(m)
}



