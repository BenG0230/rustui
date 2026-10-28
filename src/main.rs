use std::{
    io::{self, Write},
    time::Instant,
};

use rustui::{
    buffer::screenbuffer::ScreenBuffer, gfx::style::Style, input::KeyCode, renderer::display,
    terminal::Terminal,
};

fn main() -> io::Result<()> {
    let mut term = Terminal::setup_raw()?;

    let mut buff = ScreenBuffer::new(term.width, term.height);

    let mut last = Instant::now();

    'main: loop {
        let events = rustui::input::read_input()?;
        if events.len() > 0 {
            for event in events {
                match event.code {
                    KeyCode::Escape => break 'main,
                    code => print!("{:?}", code),
                }
            }
        }

        buff.write_cell(12, 11, 'o', Style::default());

        print!("\x1b[10;10H{}μs", last.elapsed().as_micros());

        last = Instant::now();

        io::stdout().flush()?;
        display(&buff)?;
        term.refresh_size()?;
        // std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
