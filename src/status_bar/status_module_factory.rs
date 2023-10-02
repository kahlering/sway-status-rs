use crate::status_bar::StatusModule;
use crate::status_modules;

pub fn create_status_module(module_conf: &serde_json::Value) -> Option<Box<dyn StatusModule>>{
    let m_type = module_conf["type"].as_number().unwrap().as_u64().unwrap();
    let mut m: Box<dyn StatusModule> = match m_type{
        0 => {match status_modules::BatteryModule::new(){
                Ok(m) => Box::new(m),
                Err(..) => return None
            }
        }
        1 => {Box::new(status_modules::DateAndTimeModule::new())},
        _ => return None,
    };
    m.configure(module_conf);
    Some(m)
}



