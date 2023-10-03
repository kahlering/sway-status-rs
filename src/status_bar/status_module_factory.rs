use crate::status_bar::StatusModule;
use crate::status_modules;

pub fn create_status_module<T>(module_type: &String, module_conf: &toml::Value) -> Option<Box<dyn StatusModule>>{
    //let m_type = module_conf["type"].as_integer().unwrap();
    let mut m: Box<dyn StatusModule> = match module_type.as_str(){
        "battery" => {match status_modules::BatteryModule::new(){
                Ok(m) => Box::new(m),
                Err(..) => return None
            }
        }
        "date_and_time" => {Box::new(status_modules::DateAndTimeModule::new())},
        "audio_volume" => {Box::new(status_modules::AudioModule::new())},
        _ => return None,
    };
    m.configure(module_conf);
    Some(m)
}



