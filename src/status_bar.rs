use std::io::{Write, Read};
use std::str;


pub trait StatusModule: Send + Sync {
    //fn get_status_string(&self) -> String;
    fn get_status_block(&mut self) -> StatusBlock;
    fn init(&mut self);
    fn handle_event(&self, event: &Event);
    fn get_instance_name(&self) -> Option<String>;
    fn get_module_name(&self) -> Option<String>;
}


#[derive(serde::Serialize)]
pub struct StatusBlock{
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<String>
}

impl StatusBlock{
    fn to_json_string(&self) -> String{
        serde_json::to_string(self).unwrap()
    }
}


#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum Button{
    RIGHT = 3,
    LEFT = 1,
    MIDDLE = 2,
    UNDEF = 4,
}

impl Default for Button{
    fn default() -> Self {
        Self::UNDEF
    }
}


#[derive(serde::Deserialize)]
pub struct Event{
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
    #[serde(default)]
    pub x: isize,
    #[serde(default)]
    pub y: isize,
    #[serde(default)]
    pub button: Button,
    #[serde(default)]
    pub relative_x: isize,
    #[serde(default)]
    pub relative_y: isize,
    #[serde(default)]
    pub output_x: isize,
    #[serde(default)]
    pub output_y: isize,
    #[serde(default)]
    pub width: isize,
    #[serde(default)]
    pub height: isize,
    #[serde(default)]
    pub modifiers: Vec<String>
}


impl Event {
    pub fn from_json(json: &str) -> serde_json::Result<Event>{
        serde_json::from_str(json)
    }

    pub fn new() -> Event{
        Event{
            name: None,
            instance: None,
            x: 0,
            y: 0,
            button: Button::LEFT,
            relative_x: 0,
            relative_y: 0,
            output_x: 0,
            output_y: 0,
            width: 0,
            height: 0,
            modifiers: Vec::new(),
        }
    }
}


pub struct StatusBar<'a, 'scope, 'env>{
    //data: std::sync::Arc<std::sync::Mutex<StatusBarData<'a>>>,
    //modules: Vec<&'a mut dyn StatusModule>,
    modules: std::sync::Arc<std::sync::Mutex<Vec<&'a mut dyn StatusModule>>>,
    free_handles: Vec<usize>,
    status_string: String,
    out: std::io::Stdout,
    scope: &'a std::thread::Scope<'scope, 'env>,
}

unsafe impl<'a,'scope, 'env> Sync for StatusBar<'a,'scope, 'env> {}
unsafe impl<'a,'scope, 'env> Send for StatusBar<'a,'scope, 'env> {}


// struct StatusBarData<'a>{
//     modules: Vec<&'a mut dyn StatusModule>,
//     status_string: String,
//     out: std::io::Stdout,
//     t: std::option::Option<std::thread::JoinHandle<()>>,
// }

impl<'a, 'scope, 'env> StatusBar<'a,'scope, 'env>{
    pub fn add_module(&mut self, module: &'a mut impl StatusModule) -> usize{
        let mut lock = self.modules.lock().expect("mutex poisoned");
        let q2: &mut Vec<&mut dyn StatusModule> = &mut *lock;
        match self.free_handles.pop(){
            Some(idx) => {q2[idx]  = module; return idx},
            None => {lock.push(module); return lock.len() - 1;}
        }
    }

    pub fn remove_module(&mut self, handle: usize){ // TODO test
        self.free_handles.push(handle);
    }

    pub fn get_status(&mut self) -> &String{
        &self.status_string
    }

    fn start_input_event_thread(&mut self)
    where
        'a: 'scope
    {
        let r: std::sync::Arc<std::sync::Mutex<Vec<&'a mut dyn StatusModule>>> = self.modules.clone();
        self.scope.spawn(move ||{
            let stdin = std::io::stdin();
            //let mut in_buf: [u8; 200] =[0; 200];
            //let mut log = std::fs::File::create("/home/k/log.txt").unwrap();
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
                                    let a = m.get_module_name();
                                    let b = a == event.name;
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

