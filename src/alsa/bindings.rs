



extern "C" {
    pub fn asd(p: *const cty::c_char) -> cty::c_int;

    pub fn initialize_alsa_lib(mix_name: *const cty::c_char , card: *const cty::c_char );

    pub fn update_volume();

    pub fn get_audio_volume() -> cty::c_long;
}


