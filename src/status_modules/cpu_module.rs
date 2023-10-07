use crate::status_bar;
use std::io::Read;
use std::io::Seek;
use num_cpus;

pub struct CPUModule{
    f_stat: std::fs::File,
    f_temp: std::fs::File,
    buf: String,
    num_cpus: usize,
    usage_vec: Vec<(usize, usize)>,
}

impl CPUModule {
    pub fn from_config(_module_conf: &toml::Value) -> Option<CPUModule>{
        let f = std::fs::File::open("/proc/stat").unwrap();
        let f_temp = std::fs::File::open("/sys/class/thermal/thermal_zone0/temp").
                           or(std::fs::File::open("/sys/class/hwmon/hwmon0/temp1_input")).unwrap();
        let num_cpus = num_cpus::get();
        //let reader = std::io::BufReader::new(f);
        Some(CPUModule { 
            f_stat: f,
            f_temp: f_temp,
            buf: String::new(),
            num_cpus: num_cpus,
            usage_vec: vec![(0,0); num_cpus],
         })
    }

    fn usage_to_char(usage: usize) -> char{
        if usage < 6{//6.25
            return '_';
        }else if usage < 19{//18.75
            return '▁';
        }else if usage < 31{//31.25
            return '▂';
        }else if usage < 44{//43.75
            return '▃';
        }else if usage < 56{//56.25
            return '▄';
        }else if usage < 67{//68.75
            return '▅';
        }else if usage < 81{//81.25
            return '▆';
        }else if usage < 93{//93.75 
            return '▇';
        }
        return '█';
    }

    fn stat_str_to_usage_string(&mut self) -> Option<String>{
        let a = self.buf.lines();
        //eprintln!("{}", self.stat_string);
        let mut cpu_string = String::from("CPU:");
        for (idx, line) in a.skip(1).take(self.num_cpus).enumerate(){
            if line.starts_with("cpu"){
                let mut split = line.split_whitespace();
                
                split.next();// skip first element
                let s1: usize= split.next()?.parse().unwrap(); // TODO handle error
                let s2: usize= split.next()?.parse().unwrap();
                let s3: usize= split.next()?.parse().unwrap();
                let rest:usize = split.fold(0, |a, i|{a + i.parse::<usize>().unwrap()}); // TODO handle error

                let work = s1 + s2 + s3;
                let total = work + rest;
                let usage_old = self.usage_vec[idx];
                let usage_new = (work, total);
                let diff_work:usize = usage_new.0 - usage_old.0;
                let diff_total:usize = (usage_new.1) - usage_old.1;
                let usage = (100 * diff_work).checked_div(diff_total).unwrap_or(100);
                cpu_string.push(Self::usage_to_char(usage));
                self.usage_vec[idx] = usage_new;
            }else{
                break;
            }
        }
        cpu_string.pop();
        Some(cpu_string)
    }

    // pub fn new() -> Option<CPUModule>{
    //     let f = std::fs::File::open("/proc/stat").unwrap();
    //     //let reader = std::io::BufReader::new(f);

    //     Some(CPUModule { 
    //         f_stat: f,
    //         total:0,
    //         used: 0,
    //         stat_string: String::new(),
    //      })
    // }
}




impl status_bar::StatusModule for CPUModule{
    fn get_instance_name(&self) -> Option<String> {
        None
    }

    fn get_module_name(&self) -> Option<String> {
        None
    }

    fn handle_event(&mut self, _event: &status_bar::Event) {
    }

    fn get_update(&mut self) -> Option<status_bar::StatusUpdate>{
        self.f_stat.seek(std::io::SeekFrom::Start(0)).unwrap();
        self.f_stat.read_to_string(&mut self.buf).unwrap();
        let mut cpu_string = self.stat_str_to_usage_string().or_else(||{eprintln!("could not generate cpu string"); None})?;
        self.buf.clear();

        self.f_temp.seek(std::io::SeekFrom::Start(0)).unwrap();
        self.f_temp.read_to_string(&mut self.buf).unwrap();
        self.buf.pop();
        let temp: usize = self.buf.parse::<usize>().unwrap() / 1000;
        cpu_string.push_str(format!(" {temp}°C").as_str());
        
        self.buf.clear();

        Some(status_bar::StatusUpdate{
            full_text: cpu_string,
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
            name: None,
            instance: None,
            separator: None,
            separator_block_width: None,
            markup: None
        })
    }
}