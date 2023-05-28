use std::{fmt, thread, time};
use std::time::Duration;

use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    MouseMove(i32, i32),
    MouseMoveR(i32, i32),
    Sleep(i32),
    Click(),
    Key(char),
}

impl Action {
    fn run(&self) {
        let mut enigo = Enigo::new();
        match self {
            Self::MouseMove(x, y) => enigo.mouse_move_to(*x, *y),
            Self::MouseMoveR(x, y) => enigo.mouse_move_relative(*x, *y),
            Self::Sleep(x) => thread::sleep(Duration::from_millis(*x as u64)),
            Self::Click() => enigo.mouse_click(MouseButton::Left),
            Self::Key(k) => enigo.key_click(Key::Layout(*k)),
            _ => todo!("unimplemented action")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macro {
    pub actions: Vec<Action>,
    name: String,
    pub repeat: bool,
    pub key: String,
}

impl Macro {
    pub fn new(name: String, repeat: bool, actions: Vec<Action>, key: String) -> Self {
        Macro { actions, name, repeat, key }
    }
    pub fn execute(&self) {
        let delay = time::Duration::from_millis(100);
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