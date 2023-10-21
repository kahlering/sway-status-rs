use sysinfo::{SystemExt, CpuExt, DiskExt};
use crate::status_bar;

macro_rules! order_to_str {
    ( $order:expr ) => {
        match $order{
            0 => "B",
            1 => "K",
            2 => "M",
            3 => "G",
            4 => "T",
            _ => ""
        }
    };
}




pub struct SysInfoModule{
    sys_info: sysinfo::System,
    disks_to_monitor: Vec<String>,
    last_disk_refresh_ts: std::time::Instant,
    last_mem_refresh_ts: std::time::Instant,
    disk_string: String,
    mem_string: String,
    show_disks: bool,
    show_mem: bool,
    show_cpu: bool,
    disk_refresh_rate_ms: u64,
    mem_refresh_rate_ms: u64,
}

impl SysInfoModule {
    pub fn from_config(_module_conf: &toml::Value) -> Result<SysInfoModule, ()>{
        let sys_info = sysinfo::System::new();
        let emty_vec:Vec<toml::Value> = Vec::new();
        let q: &Vec<toml::Value> = _module_conf.get("disks").map_or(&emty_vec, |v|{v.as_array().unwrap_or(&emty_vec)});
        let w: Vec<String> = q.into_iter().map(|v|{String::from(v.as_str().unwrap_or(""))}).collect();
        let disk_refresh_rate_ms = _module_conf.get("disk_refresh_rate_ms").and_then(|v|{v.as_integer()}).unwrap_or(60000) as u64;
        let mem_refresh_rate_ms =  _module_conf.get("mem_refresh_rate_ms").and_then(|v|{v.as_integer()}).unwrap_or(5000) as u64;
        //let disks_to_monitor:Vec<String> = vec![String::from("/dev/nvme0n1p3")];
        Ok(SysInfoModule { 
            sys_info: sys_info,
            disks_to_monitor: w,
            last_disk_refresh_ts: std::time::Instant::now() - std::time::Duration::from_millis(disk_refresh_rate_ms),
            last_mem_refresh_ts: std::time::Instant::now() - std::time::Duration::from_millis(mem_refresh_rate_ms),
            disk_string: String::new(),
            mem_string: String::new(),
            show_disks: _module_conf.get("show_disks").map_or(false, |v|{v.as_bool().unwrap_or(false)}),
            show_mem: _module_conf.get("show_mem").map_or(true, |v|{v.as_bool().unwrap_or(true)}),
            show_cpu: _module_conf.get("show_cpu").map_or(true, |v|{v.as_bool().unwrap_or(true)}),
            disk_refresh_rate_ms: disk_refresh_rate_ms,
            mem_refresh_rate_ms: mem_refresh_rate_ms,
         })
    }

    fn usage_to_char(usage: f32) -> char{
        if usage < 6.0{//6.25
            return '_';
        }else if usage < 19.0{//18.75
            return '▁';
        }else if usage < 31.0{//31.25
            return '▂';
        }else if usage < 44.0{//43.75
            return '▃';
        }else if usage < 56.0{//56.25
            return '▄';
        }else if usage < 67.0{//68.75
            return '▅';
        }else if usage < 81.0{//81.25
            return '▆';
        }else if usage < 93.0{//93.75 
            return '▇';
        }
        return '█';
    }

    

}




impl status_bar::StatusModule for SysInfoModule{
    fn get_instance_name(&self) -> Option<String> {
        None
    }

    fn get_module_name(&self) -> Option<String> {
        None
    }

    fn handle_event(&mut self, _event: &status_bar::Event) {
    }

    fn get_update(&mut self) -> Option<status_bar::StatusUpdate>{
        let mut sysinfo_string = String::new();
        sysinfo_string.reserve(200);
        let now = std::time::Instant::now();

        if self.show_disks{
            if (now - self.last_disk_refresh_ts) >= std::time::Duration::from_millis(self.disk_refresh_rate_ms){
                self.last_disk_refresh_ts = now;
                self.sys_info.refresh_disks_list();
                self.sys_info.refresh_disks();
                self.disk_string.clear();
                for disk in self.sys_info.disks() {
                    let disk_name = disk.name().to_str()?;
                    if self.disks_to_monitor.iter().any(|v|{v == disk_name}){
                        self.disk_string.push_str(disk_name);
                        self.disk_string.push(' ');
                        let mut total_space = disk.total_space() as f32;
                        let mut used_space = total_space - disk.available_space() as f32;
                        let mut order = 0;
                        while total_space > 10000.0{
                            total_space /= 1024.0;
                            used_space /= 1024.0;
                            order += 1;
                        }
                        self.disk_string.push_str(format!("{:.1}/{:.1}{} ", used_space, total_space, order_to_str!(order)).as_str());
                    }
                }
            }
            sysinfo_string.push_str(self.disk_string.as_str());
        }

        if self.show_cpu{
            self.sys_info.refresh_cpu(); // Refreshing CPU information.
            for cpu in self.sys_info.cpus() {
                sysinfo_string.push(Self::usage_to_char(cpu.cpu_usage()));
            }
            sysinfo_string.push(' ');
        }
        

        if self.show_mem{
            if (now - self.last_mem_refresh_ts) >= std::time::Duration::from_millis(self.mem_refresh_rate_ms){
                self.last_mem_refresh_ts = now;
                self.sys_info.refresh_memory();
                let used_mem = self.sys_info.used_memory() as f32 /  f32::powf(1024.0, 3.0);
                let total_mem = self.sys_info.total_memory() as f32 / f32::powf(1024.0, 3.0);
                self.mem_string = format!("Mem {0:.1}/{1:.1}G ", used_mem, total_mem);
            }
            sysinfo_string.push_str(self.mem_string.as_str());
        }

        while sysinfo_string.ends_with(' '){sysinfo_string.pop();}
        

        Some(status_bar::StatusUpdate{
            full_text: sysinfo_string,
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
}