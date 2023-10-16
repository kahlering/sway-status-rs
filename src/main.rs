mod status_bar;
mod status_modules;
mod default_config;

use std::io::BufReader;
use std::io::Read;
use toml::Value;

use crate::default_config::DEFAULT_CONFIG;

macro_rules! create_module {
    ( $status_bar:expr, $module_type:expr ,$conf:expr ) => {
        {let m = status_bar::status_module_factory::create_status_module($module_type, $conf);
        if m.is_none(){
            eprintln!("could not create module {}", $module_type);
            continue;
        }
        $status_bar.add_module(m.unwrap());}
    };
}



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
    
    let config: toml::value::Table  = toml::from_str(config_string.as_str()).unwrap();

    let refresh_rate_ms = config.get("status_bar_config").
                                and_then(|v|{v.get("refresh_rate_ms").
                                and_then(|v|{v.as_integer()})}).
                                unwrap_or_else(||{eprintln!("could not read refresh rate from config file. Using default of 1000ms"); 1000}) as u64;
    
    std::thread::scope(|s|{
        let mut status_bar = status_bar::StatusBar::new(s);

        let mut modules_conf_vec: Vec<(i64, &String, &Value)> = Vec::new(); 
        for (module_type, module_conf) in config["modules"].as_table().unwrap(){
            if module_conf.is_array(){
                for module_conf_entry in module_conf.as_array().unwrap(){
                    modules_conf_vec.push((module_conf_entry.get("status_bar_pos").and_then(|v|{v.as_integer()}).unwrap_or(-1), &module_type, &module_conf_entry));
                }
            }else{
                modules_conf_vec.push((module_conf.get("status_bar_pos").and_then(|v|{v.as_integer()}).unwrap_or(-1), &module_type, &module_conf));
            }
        }

        modules_conf_vec.sort_by(|v1, v2|{v2.0.cmp(&v1.0)});

        for (_, module_type, module_conf) in modules_conf_vec{
            let m = status_bar::status_module_factory::create_status_module(module_type, module_conf);
            if m.is_none(){
                eprintln!("could not create module {}", module_type);
                continue;
            }
            status_bar.add_module(m.unwrap());
        }

        status_bar.write_protocol_header_to_stdout().expect("failed to write protocol header to stdout"); // crash because the status bar would not work without the header
     
        loop{
            status_bar.update_status();
            if status_bar.write_status_to_stdout().is_err(){eprintln!("could not write to stdout")};
            std::thread::sleep(std::time::Duration::from_millis(refresh_rate_ms));
        }
    });
    
}
