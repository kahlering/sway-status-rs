use std::ops::Index;

const DEFAULT_CONFIG: &str = r#"[status_bar_config]
refresh_rate_ms = 1000

[modules]

[modules.battery]
refresh_rate_ms = 5000
bat_uevent_path = "/sys/class/power_supply/BAT0/uevent"
status_bar_pos = 1

[modules.date_and_time]
name = "date_and_time_module"
format = "%Y/%m/%d %T"
status_bar_pos = 0

[modules.sysinfo]
disks = ["/dev/nvme0n1p3"]
show_disks = true
show_mem = true
show_cpu = true
disk_refresh_rate_ms = 60000 
mem_refresh_rate_ms = 5000
status_bar_pos = 3

[modules.audio_volume]
status_bar_pos = 2
"#;



pub struct Config{
    c: toml::value::Table,
}


impl Config{
    
    pub fn from_file(path: &str) -> Result<Self, ()>{
        let config_string = std::fs::read_to_string(path).map_err(|_v|{()})?;
        let c = toml::from_str(config_string.as_str()).unwrap();
        Ok(Config{
            c: c,
        })
    }

    pub fn default() -> Self{
        Config { c: toml::from_str(DEFAULT_CONFIG).unwrap()}
    }

    pub fn get_refresh_rate(&self) -> u64{
        self.c.get("status_bar_config").
                                and_then(|v|{v.get("refresh_rate_ms").
                                and_then(|v|{v.as_integer()})}).
                                unwrap_or_else(||{eprintln!("could not read refresh rate from config file. Using default of 1000ms"); 1000}) as u64
    }

    pub fn get_modules(&self) -> Vec<(&String, &toml::Value)>{
        let mut modules_conf_vec: Vec<(i64, &String, &toml::Value)> = Vec::new(); 
        for (module_type, module_conf) in self.c.get("modules").unwrap().as_table().unwrap(){
            if module_conf.is_array(){
                for module_conf_entry in module_conf.as_array().unwrap(){
                    modules_conf_vec.push((module_conf_entry.get("status_bar_pos").and_then(|v|{v.as_integer()}).unwrap_or(-1), &module_type, &module_conf_entry));
                }
            }else{
                modules_conf_vec.push((module_conf.get("status_bar_pos").and_then(|v|{v.as_integer()}).unwrap_or(-1), &module_type, &module_conf));
            }
        }
        modules_conf_vec.sort_by(|v1, v2|{v2.0.cmp(&v1.0)});
        modules_conf_vec.into_iter().map(|v|{(v.1, v.2)}).collect()
    }
    
}

impl Index<&str> for Config {
    type Output = toml::Value;
    fn index(&self, i: &str) -> &toml::Value {
        &self.c[i]
    }
}

pub fn get_default_config_path() -> String{
    let home = std::env::var("HOME").unwrap();
    home + "/.config/sway_status_rust/config.toml"
}