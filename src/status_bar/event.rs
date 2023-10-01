
#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum Button{
    RIGHT = 3,
    LEFT = 1,
    MIDDLE = 2,
    UNDEF = 4,
}

impl Default for Button{
    fn default() -> Self {
        Self::UNDEF
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
    #[allow(dead_code)]
    pub fn from_json(json: &str) -> serde_json::Result<Event>{
        serde_json::from_str(json)
    }

    #[allow(dead_code)]
    pub fn new() -> Event{
        Event{
            name: None,
            instance: None,
            x: 0,
            y: 0,
            button: Button::LEFT,
            relative_x: 0,
            relative_y: 0,
            output_x: 0,
            output_y: 0,
            width: 0,
            height: 0,
            modifiers: Vec::new(),
        }
    }
}
