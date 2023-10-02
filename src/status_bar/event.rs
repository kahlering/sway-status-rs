
#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum Button{
    Right = 3,
    Left = 1,
    Middle = 2,
    ScrollUp = 4,
    ScrollDown=5,
    Undef = 100,
}

impl Default for Button{
    fn default() -> Self {
        Self::Undef
    }
}


#[derive(serde::Deserialize)]
pub struct Event{
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
    #[serde(default)]
    pub x: isize,
    #[serde(default)]
    pub y: isize,
    #[serde(default)]
    pub button: Button,
    #[serde(default)]
    pub relative_x: isize,
    #[serde(default)]
    pub relative_y: isize,
    #[serde(default)]
    pub output_x: isize,
    #[serde(default)]
    pub output_y: isize,
    #[serde(default)]
    pub width: isize,
    #[serde(default)]
    pub height: isize,
    #[serde(default)]
    pub modifiers: Vec<String>
}


impl Event {
    pub fn from_json(json: &str) -> serde_json::Result<Event>{
        serde_json::from_str(json)
    }
}
