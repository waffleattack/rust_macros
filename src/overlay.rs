//! Shows how to display a window in transparent mode.
//!
//! This feature works as expected depending on the platform. Please check the
//! [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.WindowDescriptor.html#structfield.transparent)
//! for more details.
use std::sync::mpsc::*;
use std::sync::*;


use lazy_static::lazy_static;

use types::DisplayInfo;

use crate::types;
use slint;
lazy_static! {
    pub static ref MY_CHANNEL: (Mutex<Receiver<DisplayInfo>>, Mutex<Sender<DisplayInfo>>) = {
        let (rx, tx) = channel();
        (Mutex::new(tx), Mutex::new(rx))
    };
}

slint::slint! {
    export component App inherits Window {
        background: #00000000; // A translucent color
        Text { text: "Hello World!"; }
    }
}

pub fn init_screen() {
    println!("INIT THE SCREEN");
    App::new().unwrap().run().unwrap()


}

// A unit struct to help identify the FPS UI component, since there may be many Text components
