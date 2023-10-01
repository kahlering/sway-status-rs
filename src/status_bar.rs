use std::io::{Write, Read};
use std::str;


pub trait StatusModule: Send + Sync {
    //fn get_status_string(&self) -> String;
    fn get_status_block(&mut self) -> StatusBlock;
    fn init(&mut self);
    fn handle_event(&self, event: &Event);
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
}


#[derive(serde::Deserialize)]
pub struct Event{
    pub name: Option<String>,
    pub instance: Option<String>,
    pub x: isize,
    pub y: isize,

    pub button: Button,
    pub relative_x: isize,
    pub relative_y: isize,
    #[serde(default)]
    pub output_x: isize,
    #[serde(default)]
    pub output_y: isize,
    pub width: isize,
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
    status_string: String,
    out: std::io::Stdout,
    t: std::option::Option<std::thread::JoinHandle<()>>,
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
    pub fn add_module(&mut self, module: &'a mut impl StatusModule){
        let mut lock = self.modules.lock().expect("mutex poisoned");
        lock.push(module);
        //self.modules.push(module);
    }

    pub fn get_status(&mut self) -> &String{
        &self.status_string
    }

    fn handle_event(&self, event: Event){

    }

    // fn input_thread(&self){
    //     let mut stdin = std::io::stdin();
    //     //let mut in_buf: [u8; 200] =[0; 200];
    //     let mut log = std::fs::File::create("/home/k/log.txt").unwrap();
    //     let mut buf = String::new();
    //     loop{
    //         stdin.read_line(&mut buf).expect("reading line from stdin failed");
    //         log.write_all(format!("input: {} \n\n", buf).as_bytes()).expect("failed to write to file");
    //         let event = Event::from_json(buf.as_str());
    //         buf.clear();
    //     }
    // }

    pub fn init(&mut self)
    where
        'a: 'scope
    {
        let r: std::sync::Arc<std::sync::Mutex<Vec<&'a mut dyn StatusModule>>> = self.modules.clone();
        self.scope.spawn(move ||{
            let stdin = std::io::stdin();
            //let mut in_buf: [u8; 200] =[0; 200];
            let mut log = std::fs::File::create("/home/k/log.txt").unwrap();
            let mut buf = String::new();
            loop{
                stdin.read_line(&mut buf).expect("reading line from stdin failed");
                let input = &buf.as_str()[1..];
                log.write_all(format!("input:\n {} \n\n", input).as_bytes()).expect("failed to write to file");
                let event = match Event::from_json(input)  {
                    Ok(event) => event,
                    Err(e) => {log.write_all(format!("failed to parse json {}", e).as_bytes()); Event::new()}
                };
                let mut l = r.lock().expect("mutex poisoned");
                for m in l.as_mut_slice(){
                    m.handle_event(&event);
                }
                buf.clear();
            }
        });
    }

    pub fn new(scope: &'a std::thread::Scope<'scope, 'env>) -> StatusBar<'a,'scope, 'env>{
        StatusBar { 
            modules: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            status_string: String::from(""),
            out: std::io::stdout(),
            t: None,
            scope: scope,
        }
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
        //self.out.write_all(self.status_string.as_bytes())?;
        //self.out.flush()?;
        //Ok(())
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

