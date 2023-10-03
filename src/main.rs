mod status_bar;
mod status_modules;
mod default_config;

use std::io::BufReader;
use std::io::Read;
use crate::default_config::DEFAULT_CONFIG;

fn main() {

    let mut config_string = String::new();
    let home = std::env::var("HOME").unwrap();
    let f_config = std::fs::File::open(home + "/.config/sway_status_rust/config.toml");
    match f_config{
        Ok(f) =>{
            let mut buf_reader = BufReader::new(f);
            buf_reader.read_to_string(&mut config_string).unwrap();
        },
        Err(_) => {config_string = String::from(DEFAULT_CONFIG)}
    }
    
    let config:std::collections::HashMap<String, toml::Value>  = toml::from_str(config_string.as_str()).unwrap();

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
    
}
