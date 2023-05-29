use std::{fs, process, thread};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use device_query::{DeviceQuery, DeviceState};
use device_query::keymap::Keycode;

use crate::mappings::MAPPINGS;
use crate::overlay::MY_CHANNEL;
use crate::types::*;

#[derive(Debug, PartialEq, Copy, Clone, )]
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
    last_pos: (i32, i32),
}

//    let val = map.get(&"macro1".to_owned()).unwrap();
impl Manager {
    pub fn flush(&self) {
        let path = "data.json";
        let data = serde_json::to_string(&self.macros).unwrap();
        fs::write(path, data).expect("Unable to write file");
    }
    pub fn new(macros: HashMap<String, Macro>) -> Self {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let task: Macro = rx.recv().unwrap();
                task.execute();
                tx2.send(true).unwrap()
            }
        });
        Manager { sender: tx, receiver: rx2, macros, mode: Mode::None, recorded_macro: None, last_pos: (0, 0) }
    }
    pub fn process_key(&mut self, key: Keycode) {
        let current_action: Option<String>;
        if self.receiver.try_recv().is_ok() {
            self.mode = Mode::Macro;
        }
        let mut new_mode = match key {
            Keycode::F11 => process::exit(0),
            Keycode::F1 => Mode::Macro,
            Keycode::F2 => Mode::Options,
            Keycode::F3 => Mode::None,
            _ => self.mode
        };

        let key_str = &*key.to_string();
        (new_mode, current_action) = match self.mode {
            Mode::Macro => self.mode_macro(key_str, new_mode),
            Mode::Record => self.mode_record(key, new_mode, key_str),
            Mode::Exec => (new_mode, None),
            Mode::None => (new_mode, Some(String::from(""))),
            _ => (new_mode, None)
        };
        if new_mode != self.mode {
            println!("New Mode {:#?}", new_mode);
        }
        //println!("Key : {}, Mode : {:#?}", key_str, self.mode);
        self.mode = new_mode;

        let display = DisplayInfo::from(format!("{:#?}", self.mode), current_action);
        MY_CHANNEL.1.lock().unwrap().send(display).expect("");
    }

    fn mode_record(&mut self, key: Keycode, new_mode: Mode, key_str: &str) -> (Mode, Option<String>) {
        let m = self.recorded_macro.as_mut().unwrap();

        if new_mode != Mode::Record {
            let new_macro = self.recorded_macro.take().unwrap();
            self.macros.insert(new_macro.clone().key, new_macro);
            self.flush();
            return (new_mode, Some(String::from("None")));
        } else if key == Keycode::Backspace {
            if let None = m.actions.pop() {
                self.recorded_macro.take();
                return (Mode::None, Some(String::from("Canceled Recording")));
            }
        } else {
            let (x, y) = DeviceState::new().get_mouse().coords;
            let new_action: Action = match (key, key_str.len() == 1) {
                (Keycode::F4, _) => Action::Click(),
                (Keycode::F5, _) => Action::MouseMove(x, y),
                (Keycode::F6, _) => {
                    let (x_l, y_l) = self.last_pos;
                    self.last_pos = (x, y);
                    Action::MouseMoveR(x - x_l, y - y_l)
                }
                (Keycode::F7, _) => Action::MDown(),
                (Keycode::F8, _) => Action::MUp(),

                (_x, true) => Action::KeyC(key_str.chars().next().expect("string is empty")),
                (_, _) => MAPPINGS.get(&key).unwrap_or(&Action::Sleep(10)).clone()
            };
            m.actions.push(new_action.clone());
        }
        return (new_mode, Some(format!("Key: {} Len: {} Last: {:?}", m.key, m.actions.len(), m.actions.last().unwrap_or(&Action::None()))));
    }

    fn mode_macro(&mut self, key_str: &str, mode: Mode) -> (Mode, Option<String>) {
        let mut new_mode: Mode = mode;

        if self.macros.contains_key(key_str) {
            let run_macro = self.macros.get(key_str).unwrap().clone();
            new_mode = Mode::Exec;
            self.sender.send(run_macro.clone()).unwrap();
            return (new_mode, Some(format!("Running: {}", run_macro.name)));
        } else if key_str.len() == 1 {
            new_mode = Mode::Record;
            self.recorded_macro = Some(Macro::new("PlaceHolderName".to_owned(), false, Vec::new(), key_str.to_owned()));
            println!("Recording new Macro with key {}", key_str);
            self.last_pos = DeviceState::new().get_mouse().coords;
            return (new_mode, Some(format!("Key: {} Len {} Last: None", key_str, 0)));
        };
        return (new_mode, None);
    }
}