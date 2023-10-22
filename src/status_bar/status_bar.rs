use std::io::Write;
use crate::status_bar::Event;
use crate::status_bar::StatusModule;
use crate::status_bar::StatusUpdate;


#[derive(serde::Serialize)]
struct SwayStatusUpdate{
    name: String,
    #[serde(flatten)]
    update: StatusUpdate
}

#[derive(serde::Deserialize)]
struct SwayStatusEvent{
    name: String,
    #[serde(flatten)]
    event: Event,
}


pub struct StatusBar{
    modules: std::sync::Arc<std::sync::Mutex<Vec<Box<dyn StatusModule>>>>,
    module_update_string_buffer: Vec<String>,
    status_string: String,
    out: std::io::Stdout,
}

impl StatusBar{
    pub fn add_module(&mut self, module: Box<dyn StatusModule>){
        let mut lock = self.modules.lock().expect("mutex poisoned");
        lock.push(module);
        self.module_update_string_buffer.push(String::from("{}"));
    }

    
    pub fn start_input_event_thread<'a,'scope, 'env>(&mut self, scope: &'a std::thread::Scope<'scope, 'env>)
    where 
        'a: 'scope
    {
        let arc_modules_clone: std::sync::Arc<std::sync::Mutex<Vec<Box<dyn StatusModule>>>> = self.modules.clone();

        scope.spawn(move ||{
            
            let stdin = std::io::stdin();
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("reading line from stdin failed"); // first line is just "["
            loop{
                buf.clear();
                stdin.read_line(&mut buf).expect("reading line from stdin failed");
                match buf.find('{'){
                    Some(start_idx) =>{
                        let input = &buf.as_str()[start_idx..];
                        match serde_json::from_str::<SwayStatusEvent>(input) {
                            Ok(swayevent) => {
                                let mut l = arc_modules_clone.lock().expect("mutex poisoned");
                                let Ok(module_idx) = swayevent.name.parse::<usize>() else {eprintln!("failed to parse event name to index"); continue;};
                                let m = l.get_mut(module_idx).unwrap();
                                m.handle_event(&swayevent.event);                                 
                            },
                            Err(e) => {
                                eprintln!("failed to parse json: {}", e);
                            }
                        };
                        
                    }
                    None => {
                        eprintln!("Error while parsing input: could not find {{ in line: {}", buf);
                    }
                }
            }
        });
    }

    pub fn new() -> Self
    {
        StatusBar { 
            modules: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            status_string: String::from(""),
            out: std::io::stdout(),
            module_update_string_buffer: Vec::new(),
        }
    }

    pub fn update_status(&mut self){
        self.status_string.clear();
        self.status_string.push_str("[");
        let mut lock = self.modules.lock().expect("mutex poisoned");
        for (i, m) in lock.iter_mut().enumerate(){
            match m.get_update(){
                Some(update) =>{
                    let u = SwayStatusUpdate{
                        name: i.to_string(),
                        update: update,
                    };
                    self.module_update_string_buffer[i] = serde_json::to_string(&u).unwrap();
                },
                None => {}
            }
            self.status_string.push_str(self.module_update_string_buffer[i].as_str());
            self.status_string.push(',');
        }
        self.status_string.push_str("],\n");
    }

    pub fn write_status_to_stdout(&mut self) -> std::io::Result<()>{
        Self::write_to_stdout(&mut self.out, self.status_string.as_bytes())
    }

    pub fn write_protocol_header_to_stdout(&mut self) -> std::io::Result<()>{
        Self::write_to_stdout(&mut self.out,"{\"version\":1,\"stop_signal\":10,\"cont_signal\":12,\"click_events\":true}\n[\n".as_bytes())
    }

    fn write_to_stdout(out: &mut std::io::Stdout, buf: &[u8]) -> std::io::Result<()>{
        out.write_all(buf)?;
        out.flush()?;
        Ok(())
    }

}

