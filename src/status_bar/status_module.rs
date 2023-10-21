use crate::status_bar::Event;
use crate::status_bar::StatusUpdate;
pub trait StatusModule: Send + Sync{
    /// Return Some(StatusUpdate) or None  if no update is available
    fn get_update(&mut self) -> Option<StatusUpdate>;
    fn handle_event(&mut self, event: &Event);
}