use std::{fs, process};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use device_query::{DeviceQuery, DeviceState};
use device_query::keymap::Keycode;

use crate::types::{Action, Macro};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Macro,
    Options,
    Exec,
    Record,
    None,
}

pub struct Manager {
    sender: Sender<Macro>,
    receiver: Receiver<bool>,
    macros: HashMap<String, Macro>,
    mode: Mode,
    recorded_macro: Option<Macro>,
}

//    let val = map.get(&"macro1".to_owned()).unwrap();
impl Manager {
    pub fn flush(&self) {
        let path = "data.json";
        let data = serde_json::to_string(&self.macros).unwrap();
        fs::write(path, data).expect("Unable to write file");
    }
    pub fn new(sender: Sender<Macro>, receiver: Receiver<bool>, macros: HashMap<String, Macro>) -> Self {
        Manager { sender, receiver, macros, mode: Mode::None, recorded_macro: None }
    }
    pub fn process_key(&mut self, key: Keycode) {
        let mut new_mode = match key {
            Keycode::Escape => process::exit(0),
            Keycode::F1 => Mode::Macro,
            Keycode::F2 => Mode::Options,
            Keycode::F3 => Mode::None,
            _ => self.mode
        };
        if new_mode != self.mode {
            println!("New Mode {:#?}", new_mode)
        }
        let key_str = &*format!("{}", key);
        println!("Key : {}, Mode : {:#?}", key_str, self.mode);
        if self.mode == Mode::Macro {
            if self.macros.contains_key(key_str) {
                println!("executing macro");
                new_mode = Mode::Exec;
                self.sender.send(self.macros.get(key_str).unwrap().clone()).unwrap();
            } else if key_str.len() == 1 {
                new_mode = Mode::Record;
                self.recorded_macro = Some(Macro::new("PlaceHolderName".to_owned(), false, Vec::new(), key_str.to_owned()));
                println!("Recording new Macro with key {}", key_str)
            }
        }
        if self.mode == Mode::Exec {
            println!("Checking if done");
        }
        if self.mode == Mode::Record {
            if new_mode != Mode::Record {
                let new_macro = self.recorded_macro.take().unwrap();
                self.macros.insert(new_macro.clone().key, new_macro);
                self.flush();
            } else {
                let (x, y) = DeviceState::new().get_mouse().coords;
                let new_action = match (key, key_str.len() == 1) {
                    (Keycode::F4, _) => Action::MouseMove(x, y),
                    (Keycode::F5, _) => Action::Click(),

                    (_x, true) => Action::Key(key_str.chars().next().expect("string is empty")),
                    (_, _) => todo!("uhhh idk")
                };
                self.recorded_macro.as_mut().unwrap().actions.push(new_action);
            }
        }

        self.mode = new_mode;
        match self.receiver.try_recv() {
            Err(_) => println!("FAiled"),
            Ok(_) => {
                println!("Recieved Done");
                self.mode = Mode::Macro;
                self.process_key(key)
            }
        }
    }
}