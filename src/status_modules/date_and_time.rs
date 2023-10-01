use crate::status_bar;
use chrono::{DateTime, Local};

const MODULE_NAME: &str = "date_and_time_module";
const INSTANCE_NAME: &str = "date_and_time_module_instance_1";


pub struct DateAndTimeModule{

}


impl status_bar::StatusModule for DateAndTimeModule{
    fn get_instance_name(&self) -> Option<String> {
        Some(String::from(INSTANCE_NAME))
    }

    fn get_module_name(&self) -> Option<String> {
        Some(String::from(MODULE_NAME))
    }

    fn handle_event(&self, _event: &status_bar::Event) {
    }

    fn get_status_block(&mut self) -> status_bar::StatusBlock{
        let now: DateTime<Local> = Local::now();
        status_bar::StatusBlock{
            full_text: String::from(now.format("%Y/%m/%d %T").to_string()),
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


impl DateAndTimeModule{
    pub fn new() -> DateAndTimeModule{
        DateAndTimeModule{}
    }
}