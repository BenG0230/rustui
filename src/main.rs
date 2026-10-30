use std::{
    io::{self},
    time::Instant,
};

use rustui::{Engine, KeyCode, Style};

fn main() -> io::Result<()> {
    let mut engine = Engine::new()?;

    'main: loop {
        engine.clear_buffer();

        let events = engine.read_inputs()?;
        for event in events {
            match event.code {
                KeyCode::Escape => break 'main,
                _ => {}
            }
        }

        engine.draw_line(10, 10, 15, 15, 'o', Style::default());

        engine.draw()?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
