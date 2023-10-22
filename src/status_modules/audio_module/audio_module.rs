use crate::status_bar;

use super::bindings::{self, set_audio_volume};


pub struct AudioModule{
    volume: i32,
}

impl AudioModule {
    pub fn from_config(_module_conf: &toml::Value) -> Result<AudioModule, ()>{
        Ok(AudioModule{
            volume: -1000,
        })
    }
}


impl status_bar::StatusModule for AudioModule {

    fn get_update(&mut self) -> Option<crate::status_bar::StatusUpdate> {
        let new_vol: cty::c_int;
        unsafe{
            new_vol = bindings::get_audio_volume();
        }

        self.volume = new_vol;
        let text: String;
        match new_vol{
            -2 => {text = String::from("Vol M")},
            -1 => {text = String::new()},
            _ => {text = String::from(format!("Vol {new_vol}"))}
        }
        
        Some(status_bar::StatusUpdate{
            full_text: text,
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
            separator: None,
            separator_block_width: None,
            markup: None
        })
    }

    fn handle_event(&mut self, _event: &crate::status_bar::Event) {
        match _event.button{
            status_bar::Button::ScrollDown => {
                let new_vol = std::cmp::max(0, self.volume - 5);
                unsafe{set_audio_volume(new_vol)};
                self.volume = new_vol;
            },
            status_bar::Button::ScrollUp => {
                let new_vol = std::cmp::min(100, self.volume + 5);
                unsafe{set_audio_volume(new_vol)};
                self.volume = new_vol;
            },
            _ => {}
        }
    }

    
}