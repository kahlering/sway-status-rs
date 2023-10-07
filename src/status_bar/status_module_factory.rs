use crate::status_bar::StatusModule;
use crate::status_modules;

pub fn create_status_module(module_type: &String, module_conf: &toml::Value) -> Option<Box<dyn StatusModule>>{
    let m: Box<dyn StatusModule> = match module_type.as_str(){
        "battery" => {Box::new(status_modules::BatteryModule::from_config(module_conf)?)},
        "date_and_time" => {Box::new(status_modules::DateAndTimeModule::from_config(module_conf)?)},
        "audio_volume" => {Box::new(status_modules::AudioModule::from_config(module_conf)?)},
        "cpu" => {Box::new(status_modules::CPUModule::from_config(module_conf)?)},
        _ => return None,
    };
    Some(m)
}



