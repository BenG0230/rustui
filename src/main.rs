use std::io::{self, Write};

use rustui::{input::KeyCode, terminal::Terminal};

fn main() -> io::Result<()> {
    let term = Terminal::setup_raw()?;

    'main: loop {
        let events = rustui::input::read_input()?;
        if events.len() > 0 {
            for event in events {
                match event.code {
                    KeyCode::Escape => break 'main,
                    code => print!("{:?} ", code),
                }
            }
        }
        io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
