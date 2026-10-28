use std::io::{self, Write};

use rustui::{
    input::raw::{input_available, read_byte},
    terminal::Terminal,
};

fn main() -> io::Result<()> {
    let term = Terminal::setup_raw()?;

    loop {
        let events = rustui::input::keyboard::read_input()?;
        if events.len() > 0 {
            for event in events {
                print!("{:?} ", event.code);
            }
        }
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
