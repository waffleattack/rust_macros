use std::collections::HashMap;

use device_query::Keycode;

use lazy_static::lazy_static;

use crate::types::Action;
use crate::types::Action::*;

lazy_static! {
pub static ref MAPPINGS: HashMap<Keycode, Action> = HashMap::from([
    (Keycode::Space, KeyC(' ')),
    (Keycode::Key0, KeyC('0')),
    (Keycode::Key1, KeyC('1')),
    (Keycode::Key2, KeyC('2')),
    (Keycode::Key3, KeyC('3')),
    (Keycode::Key4, KeyC('4')),
    (Keycode::Key5, KeyC('5')),
    (Keycode::Key6, KeyC('6')),
    (Keycode::Key7, KeyC('7')),
    (Keycode::Key8, KeyC('8')),
    (Keycode::Key9, KeyC('9')),
]);
}