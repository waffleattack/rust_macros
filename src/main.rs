use std::{fs, thread};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, mpsc, Mutex};

use device_query::{DeviceEvents, DeviceState, keymap::Keycode};

use types::*;

use crate::manager::Manager;

mod types;
mod manager;


fn main() {

    //use crossbeam_queue::SegQueue;
    let device_buffer_head: Arc<Mutex<VecDeque<Keycode>>> = Arc::new(Mutex::new(VecDeque::new()));
    let buffer_in = device_buffer_head.clone();
    let buffer_out = device_buffer_head.clone();
    let device_state = DeviceState::new();

    let map: HashMap<String, Macro> = match fs::read_to_string("data.json") {
        Err(_) => HashMap::new(),
        Ok(c) => serde_json::from_str(&c).expect("Data.jsin Should be properly formatted")
    };


    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        println!("gett");
        loop {
            let task: Macro = rx.recv().unwrap();
            println!("Executing {}", task);
            task.execute();
            println!("We done biiiitch");
            tx2.send(true).unwrap()
        }
    });

    let mut manager = Manager::new(tx, rx2, map);
    let _guard = device_state.on_key_down(move |key| {
        buffer_in.lock().unwrap().push_back(*key);
    });
    loop {
        while buffer_out.lock().unwrap().len() > 0 {
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
