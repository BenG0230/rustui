mod decoder;
mod keyboard;
mod raw;

pub use decoder::{KeyCode, KeyEvent};
pub(crate) use keyboard::read_input;
