use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::{Arc, Mutex};

use device_query::{DeviceEvents, DeviceState, keymap::Keycode};

use types::*;

use crate::manager::Manager;

mod types;
mod manager;
mod mappings;


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
    loop {
        if !buffer_out.lock().unwrap().is_empty() {
            manager.process_key(buffer_out.lock().unwrap().pop_front().unwrap())
        }
    }


    /*let q = SegQueue::new();
    v1.iter().for_each(|x| q.push(x));
    for x in q {

    }
    println!("{:?}",q)
    */
}
