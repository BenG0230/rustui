use std::io;

use super::decoder::{Decoder, KeyEvent};
use super::raw::{input_available, read_bytes};

pub fn read_input() -> io::Result<Vec<KeyEvent>> {
    let mut decoder = Decoder::new();
    let mut buf = [0u8; 64];

    while input_available()? {
        let n = read_bytes(&mut buf)?;

        decoder.push_bytes(&buf[..n]);
    }

    Ok(decoder.get_events())
}
