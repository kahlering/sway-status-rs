use crate::status_bar;
use chrono::{DateTime, Local};


pub struct DateAndTimeModule{
    format: String,
}


impl status_bar::StatusModule for DateAndTimeModule{

    fn get_instance_name(&self) -> Option<String> {
        None
    }

    fn get_module_name(&self) -> Option<String> {
        None
    }

    fn handle_event(&mut self, _event: &status_bar::Event) {
    }

    fn get_update(&mut self) -> Option<status_bar::StatusUpdate>{
        let now: DateTime<Local> = Local::now();
        return Some(status_bar::StatusUpdate{
            full_text: String::from(now.format(self.format.as_str()).to_string()),
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
            name: None,
            instance: None,
            separator: None,
            separator_block_width: None,
            markup: None
        })
    }
}


impl DateAndTimeModule{
    pub fn from_config(module_conf: &toml::Value) -> Option<DateAndTimeModule>{
        let format = module_conf.get("format").and_then(|v|{v.as_str()}).unwrap_or_else(||{eprintln!("DateAndTimeModule: could not read format from config. Using default."); "%Y/%m/%d %T"});

        Some(DateAndTimeModule{
            format: String::from(format),
        })
    }
}