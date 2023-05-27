use std::{fmt, thread, time};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    MouseMove(i32, i32),
    Sleep(i32),
    Exit(),
    MouseKey(char),
}

impl Action {
    fn run(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macro {
    actions: Vec<Action>,
    name: String,
    pub repeat: bool,
}

impl Macro {
    pub fn new(name: String, repeat: bool, actions: Vec<Action>) -> Self {
        Macro { actions, name, repeat }
    }
    pub fn execute(&self) {
        let delay = time::Duration::from_millis(1000);
        for task in self.actions.iter() {
            task.run();
            thread::sleep(delay);
        }
    }
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Repeating {}, Length {}", self.name, self.repeat, self.actions.len())
    }
}