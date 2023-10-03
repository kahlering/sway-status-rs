extern "C" {
    pub fn get_audio_volume() -> cty::c_int;
    pub fn set_audio_volume(volume: cty::c_int);
}


