pub mod alsa;
mod status_bar;
mod status_modules;

use std::io::{Read, Write};
use std::str;

// pub unsafe extern "C" fn fcntl(fd: cty::c_int, cmd: cty::c_int, mut args: ...) -> cty::c_int {
    
// }



extern "C" {
    fn set_stdin_nonblocking();
}


use std::{
    sync::{Arc, Mutex},
    thread,
};


struct Stats<'a>{
    test: Arc<Mutex<&'a String>>,
}
impl<'a> Stats<'a> {
    fn add_stats(&mut self, _other: &Stats) {}
}

fn main() {


    // //let shared_stats = Arc::new(Mutex::new(Stats{}));
    // let q1 = String::from("asd");
    // let stats = Stats{test: Arc::new(Mutex::new(&q1))};

    // let threads = 5;
    // for _ in 0..threads {
    //     //let my_stats = stats.test.clone();
    //     let my_stats = stats.test;
    //     thread::spawn(move || {
    //         let mut shared = my_stats.lock().unwrap();
    //         shared.push_str("asd");
    //     });
    //     // Note: Immediately joining, no multithreading happening!
    //     // THIS WAS A LIE, see below
    // }

    // loop{
    //     let mut shared = stats.test.lock().unwrap();
    //     shared.push_str("asd2");
    // }



    //unsafe{
    //    set_stdin_nonblocking();
    //}
    
    //let mut stdin = std::io::stdin();

    //let mut in_buf: [u8; 200] =[0; 200];
    let mut bat_mod = status_modules::bat::BatteryModule::new().expect("failed to init battery module");

    std::thread::scope(|s|{
        let mut status_bar = status_bar::StatusBar::new(s);
        
        status_bar.add_module(&mut bat_mod);

        status_bar.write_protocol_header_to_stdout().expect("failed to write protocol header to stdout");
        status_bar.init();

        //let mut log = std::fs::File::create("/home/k/log.txt").unwrap();

        loop{
            //println!("{}", status_bar.get_status_string());
            status_bar.update_status();
            status_bar.write_status_to_stdout().expect("writing to stdout failed");
            //let bytes_read = stdin.read(&mut in_buf).unwrap_or(0);
            //log.write_all(format!("bytes read {}\n input: {} \n\n", bytes_read, str::from_utf8(&in_buf[0..bytes_read]).unwrap()).as_bytes()).unwrap();
            //eprintln!("bytes read {}", bytes_read);
            //eprintln!("input: {}", str::from_utf8(&in_buf[0..bytes_read]).unwrap());
            std::thread::sleep(std::time::Duration::from_secs(1));
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
