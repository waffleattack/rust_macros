use std::{fs, thread};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, mpsc, Mutex};

use device_query::{DeviceEvents, DeviceState, keymap::Keycode};
use types::*;
use types::Action::*;

use crate::manager::Manager;

mod types;
mod manager;


fn main() {
    let path = "data.json";
    //use crossbeam_queue::SegQueue;
    let device_buffer_head: Arc<Mutex<VecDeque<Keycode>>> = Arc::new(Mutex::new(VecDeque::new()));
    let buffer_in = device_buffer_head.clone();
    let buffer_out = device_buffer_head.clone();
    let device_state = DeviceState::new();

    let mut map: HashMap<String, Macro> = match fs::read_to_string(path) {
        Err(_) => HashMap::new(),
        Ok(c) => serde_json::from_str(&c).unwrap()
    };
    let v1: Vec<Action> = vec![MouseMove(10, 10), Sleep(10), Exit(), MouseKey('x')];
    let macro1 = Macro::new("Test Macro".to_owned(), false, v1);
    if !map.contains_key("macro1") {
        map.insert(
            "macro1".to_owned(),
            macro1,
        );
    };
    let data = serde_json::to_string(&map).unwrap();
    fs::write(path, data).expect("Unable to write file");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("gett");
        loop {
            let task: Macro = rx.recv().unwrap();
            println!("Executing {}", task);
            task.execute();
        }
    });
    println!("{:#?}", map);
    println!("{:#?}", map.get("Q"));
    let mut manager = Manager::new(tx, map);
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
