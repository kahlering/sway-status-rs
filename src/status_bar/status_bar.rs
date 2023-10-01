use std::io::{Write};
use crate::status_bar::Event;
use crate::status_bar::StatusModule;

pub struct StatusBar<'a, 'scope, 'env>{
    modules: std::sync::Arc<std::sync::Mutex<Vec<Box<dyn StatusModule>>>>,
    free_handles: Vec<usize>,
    status_string: String,
    out: std::io::Stdout,
    scope: &'a std::thread::Scope<'scope, 'env>,
}

unsafe impl<'a,'scope, 'env> Sync for StatusBar<'a,'scope, 'env> {}
unsafe impl<'a,'scope, 'env> Send for StatusBar<'a,'scope, 'env> {}

impl<'a, 'scope, 'env> StatusBar<'a,'scope, 'env>{
    pub fn add_module(&mut self, module: impl StatusModule) -> usize{
        let mut lock = self.modules.lock().expect("mutex poisoned");
        let q2: &mut Vec<Box<dyn StatusModule>> = &mut *lock;
        match self.free_handles.pop(){
            Some(idx) => {q2[idx]  = Box::new(module); return idx},
            None => {lock.push(Box::new(module)); return lock.len() - 1;}
        }
    }

    #[allow(dead_code)]
    pub fn remove_module(&mut self, handle: usize) -> Box<dyn StatusModule>{ // TODO test, free handle twice = error, free unused handle = error
        self.free_handles.push(handle);
        let mut lock = self.modules.lock().expect("mutex poisoned");
        let q2: &mut Vec<Box<dyn StatusModule>> = &mut *lock;
        q2.remove(handle)
    }

    #[allow(dead_code)]
    pub fn get_status(&mut self) -> &String{
        &self.status_string
    }

    fn start_input_event_thread(&mut self)
    where
        'a: 'scope
    {
        let r: std::sync::Arc<std::sync::Mutex<Vec<Box<dyn StatusModule>>>> = self.modules.clone();
        self.scope.spawn(move ||{
            let stdin = std::io::stdin();
            let mut buf = String::new();
            loop{
                stdin.read_line(&mut buf).expect("reading line from stdin failed");
                match buf.find('{'){
                    Some(start_idx) =>{
                        let input = &buf.as_str()[start_idx..];
                        eprintln!("start idx: {} buf:\n {} \n\n", start_idx, buf);
                        match Event::from_json(input)  {
                            Ok(event) => {
                                if event.name.as_ref().is_none(){
                                    continue
                                }
                                let mut l = r.lock().expect("mutex poisoned");
                                for m in l.as_mut_slice(){
                                    if m.get_module_name() == event.name && m.get_instance_name() == event.instance{ //TODO check if == works as expected on option
                                        m.handle_event(&event);
                                    }
                                }                                   
                            },
                            Err(e) => {
                                eprintln!("failed to parse json: {}", e);
                            }
                        };
                        
                    }
                    None => {
                        eprintln!("could not find {{ in line {}", buf);
                    }
                }
                buf.clear();
            }
        });
    }

    pub fn new(scope: &'a std::thread::Scope<'scope, 'env>) -> StatusBar<'a,'scope, 'env>
    where
    'a: 'scope
    {
        let mut r = StatusBar { 
            modules: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            status_string: String::from(""),
            out: std::io::stdout(),
            scope: scope,
            free_handles: Vec::new(),
        };
        r.start_input_event_thread();
        r
    }

    pub fn update_status(&mut self){
        self.status_string.clear();
        self.status_string.push_str("[");
        let mut lock = self.modules.lock().expect("mutex poisoned");
        for m in lock.as_mut_slice(){
            self.status_string.push_str(m.get_status_block().to_json_string().as_str());
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

