mod decoder;
mod keyboard;
mod raw;

pub use decoder::{KeyCode, KeyEvent};
pub use keyboard::read_input;
