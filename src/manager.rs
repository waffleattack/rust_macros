use std::collections::HashMap;
use std::process;
use std::sync::mpsc::Sender;

use device_query::keymap::Keycode;

use crate::types::Macro;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Macro,
    Options,
    Record,
    StartRecord,
    None,
}

pub struct Manager {
    sender: Sender<Macro>,
    macros: HashMap<String, Macro>,
    mode: Mode,
}

//    let val = map.get(&"macro1".to_owned()).unwrap();
impl Manager {
    pub fn new(sender: Sender<Macro>, macros: HashMap<String, Macro>) -> Self {
        Manager { sender, macros, mode: Mode::None }
    }
    pub fn process_key(&mut self, key: Keycode) {
        let new_mode = match key {
            Keycode::Escape => process::exit(0),
            Keycode::BackSlash => Mode::Macro,
            Keycode::RightBracket => Mode::Options,
            Keycode::LeftBracket => Mode::None,
            _ => self.mode
        };
        if new_mode != self.mode {
            println!("New Mode {:#?}", new_mode)
        }
        let key_str = &*format!("{}", key);
        println!("{}", key_str);
        if self.mode == Mode::Macro && self.macros.contains_key(&*format!("{}", key)) {
            println!("executing macro");
            self.sender.send(self.macros.get(&*format!("{}", key)).unwrap().clone()).unwrap()
        }
        self.mode = new_mode;
    }
}