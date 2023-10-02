pub mod alsa;
mod status_bar;
mod status_modules;

use std::io::BufReader;
use std::io::Read;

fn main() {

    let home = std::env::var("HOME").unwrap();
    let f_config = std::fs::File::open(home + "/.config/sway_status_rust/config.toml").unwrap();
    let mut buf_reader = BufReader::new(f_config);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    //let config: std::collections::HashMap<String, serde_json::Value> = serde_json::from_str(contents.as_str()).unwrap();
    let config:std::collections::HashMap<String, toml::Value>  = toml::from_str(contents.as_str()).unwrap();

    let refresh_rate_ms = config["status_bar_config"]["refresh_rate_ms"].as_integer().unwrap_or_else(||{eprint!("could not read refresh rate from config file. Using default of 1000ms"); 1000}) as u64;
    
    std::thread::scope(|s|{
        let mut status_bar = status_bar::StatusBar::new(s);

        for (module_type, module_conf) in config["modules"].as_table().unwrap(){
            let m = status_bar::status_module_factory::create_status_module(module_type, module_conf).unwrap();
            status_bar.add_module(m);
        } 

        
        status_bar.write_protocol_header_to_stdout().expect("failed to write protocol header to stdout");
     

        loop{
            status_bar.update_status();
            status_bar.write_status_to_stdout().expect("writing to stdout failed");
            std::thread::sleep(std::time::Duration::from_millis(refresh_rate_ms));
        }
    });
    
    


    // unsafe{

    //     let mix_name: *const cty::c_char = "Speaker\0".as_ptr() as *const i8;
    //     let card: *const cty::c_char = "hw:0\0".as_ptr() as *const i8;
    //     alsa::bindings::initialize_alsa_lib(mix_name, card);

    //     let s = String::from("asd");

    //     let mut v: Vec<char> = vec!['छ';200];
    //     let test_s = "asd छ sdfsdf";


    //     let mut arr: [u8; 200] = [0; 200];

    //     let t2 = &arr[0..2];

    //     let t = &s.as_bytes()[0..2];

    //     let s3 = "asd";

    //     let s2: &String = &s;

    //     let mut a = String::from_iter(&v);

        

        

    //     loop{
    //         println!("{}", status_bar.get_status_string());

    //         //alsa::bindings::asd(mix_name);
    //         //alsa::bindings::update_volume();
    //         //let vol = alsa::bindings::get_audio_volume();
    //         //println!("vol {vol}");
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //     }
    //     //let a: i32 = alsa::bindings::asd();
    //     //b = alsa::bindings::asd(mix_name);
        
    // }

    //println!("b {b}");

    
    
}
