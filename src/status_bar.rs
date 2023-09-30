use std::io::{Write, Read};
use std::str;
pub trait StatusModule {
    //fn get_status_string(&self) -> String;
    fn get_status_block(&mut self) -> StatusBlock;
    fn init(&mut self);
    fn click_event(&self);
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


#[derive(serde::Deserialize)]
pub enum Button{
    RIGHT,
    LEFT,
    MIDDLE,
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
    pub output_x: isize,
    pub output_y: isize,
    pub width: isize,
    pub height: isize,
    pub modifiers: Vec<String>
}


impl Event {
    pub fn from_json(json: &str) -> Event{
        serde_json::from_str(json).unwrap()
    }
}


pub struct StatusBar<'a>{
    modules: Vec<&'a mut dyn StatusModule>,
    status_string: String,
    out: std::io::Stdout,
    t: std::option::Option<std::thread::JoinHandle<()>>,
}



impl<'a> StatusBar<'a>{
    pub fn add_module(&mut self, module: &'a mut impl StatusModule){
        self.modules.push(module);
    }

    pub fn get_status(&mut self) -> &String{
        &self.status_string
    }

    fn handle_event(&self, event: Event){

    }

    fn input_thread(&self){
        let mut stdin = std::io::stdin();
        //let mut in_buf: [u8; 200] =[0; 200];
        let mut log = std::fs::File::create("/home/k/log.txt").unwrap();
        let mut buf = String::new();
        loop{
            stdin.read_line(&mut buf).expect("reading line from stdin failed");
            log.write_all(format!("input: {} \n\n", buf).as_bytes()).expect("failed to write to file");
            let event = Event::from_json(buf.as_str());
            buf.clear();
        }
    }

    pub fn init(this: std::sync::Arc<std::sync::Mutex<Self>>){
        this.lock().expect("mutex is poisoned");//.acti = true;
        std::thread::spawn(move ||{
            let lock = this.lock().expect("mutex asd");
        });
        //this.t = Some(std::thread::spawn(||{self.input_thread()}));
    }

    pub fn new() -> StatusBar<'a>{
        StatusBar { 
            modules: Vec::new(),
            status_string: String::from(""),
            out: std::io::stdout(),
            t: None,
        }
    }

    pub fn update_status(&mut self){
        self.status_string.clear();
        self.status_string.push_str("[");
        for m in &mut self.modules{
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

