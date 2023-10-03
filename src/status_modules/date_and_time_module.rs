use crate::status_bar;
use chrono::{DateTime, Local};


pub struct DateAndTimeModule{
    name: Option<String>,
    instance: Option<String>,
    format: String,
}


impl status_bar::StatusModule for DateAndTimeModule{

    fn configure(&mut self, module_conf: &toml::Value) {
        let name = module_conf["name"].as_str();
        match name{
            None => {eprint!("could not read name from config file for date and time module"); return;},
            Some(s) => {self.name = Some(String::from(s))}
        }
        let format = module_conf["format"].as_str();
        match format{
            None => {eprint!("could not read format from config file for date and time module"); return;},
            Some(s) => {self.format = String::from(s)}
        }
    }

    fn get_instance_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_module_name(&self) -> Option<String> {
        self.instance.clone()
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
            name: self.name.clone(),
            instance: self.instance.clone(),
            separator: None,
            separator_block_width: None,
            markup: None
        })
    }
}


impl DateAndTimeModule{
    pub fn new() -> DateAndTimeModule{
        DateAndTimeModule{
            name: None,
            instance: None,
            format: String::from("%Y/%m/%d %T"),
        }
    }
}