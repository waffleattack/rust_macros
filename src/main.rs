use std::{fs, thread};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use device_query::{DeviceEvents, DeviceState, keymap::Keycode};

use crate::manager::Manager;
use crate::types::*;

mod types;
mod manager;
mod mappings;
mod overlay;

fn main() {
    let buffer_head: Arc<Mutex<VecDeque<Keycode>>> = Default::default();
    let buffer_in = buffer_head.clone();
    let buffer_out = buffer_head.clone();
    let device_state = DeviceState::new();
    let map: HashMap<String, Macro> = fs::read_to_string("data.json")
        .map_or_else(
            |_e| HashMap::new(),
            |c| serde_json::from_str(&c).expect("Data.json Should be properly formatted"),
        );
    let mut manager = Manager::new(map);
    let _guard = device_state.on_key_down(move |key| {
        buffer_in.lock().unwrap().push_back(*key);
    });
    thread::spawn(move || {
        loop {
            if !buffer_out.lock().unwrap().is_empty() {
                manager.process_key(buffer_out.lock().unwrap().pop_front().unwrap())
            }
        }
    });
    overlay::init_screen()
}
