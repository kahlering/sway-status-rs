mod status_bar;
mod status_modules;
mod config;



fn main() {

    let config = config::Config::from_file(config::get_default_config_path().as_str()).unwrap_or(config::Config::default());
    let refresh_rate_ms = config.get_refresh_rate();
    
    
    let mut status_bar = status_bar::StatusBar::new();

    let modules_conf_vec = config.get_modules();

    for (module_type, module_conf) in modules_conf_vec{
        let Some(m) = status_bar::status_module_factory::create_status_module(module_type, module_conf) else{
            eprintln!("could not create module {}", module_type);
            continue;
        };
        status_bar.add_module(m);
    }

    std::thread::scope(|thread_scope|{
        status_bar.start_input_event_thread(thread_scope);

        status_bar.write_protocol_header_to_stdout().expect("failed to write protocol header to stdout"); // crash because the status bar would not work without the header
     
        loop{
            status_bar.update_status();
            if status_bar.write_status_to_stdout().is_err(){eprintln!("could not write to stdout")};
            std::thread::sleep(std::time::Duration::from_millis(refresh_rate_ms));
        }
    });
    
}
