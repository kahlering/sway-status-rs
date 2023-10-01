use crate::status_bar::Event;
use crate::status_bar::StatusBlock;
pub trait StatusModule: Send + Sync + 'static {
    //fn get_status_string(&self) -> String;
    fn get_status_block(&mut self) -> StatusBlock;
    fn handle_event(&self, event: &Event);
    fn get_instance_name(&self) -> Option<String>;
    fn get_module_name(&self) -> Option<String>;
}