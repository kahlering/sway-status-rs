use crate::status_bar::Event;
use crate::status_bar::StatusUpdate;
pub trait StatusModule: Send + Sync + 'static {
    /// Return Some(StatusUpdate) or None  if no update is available
    fn get_update(&mut self) -> Option<StatusUpdate>;
    fn handle_event(&mut self, event: &Event);
    fn get_instance_name(&self) -> Option<String>;
    fn get_module_name(&self) -> Option<String>;
    fn configure(&mut self, module_conf: &toml::Value);
    fn from_config<T: StatusModule>(module_conf: &toml::Value) -> Option<T>;
}