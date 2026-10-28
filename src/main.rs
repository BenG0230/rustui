use std::{
    io::{self, Write},
    time::Instant,
};

use rustui::{engine::Engine, gfx::style::Style, input::KeyCode};

fn main() -> io::Result<()> {
    let mut engine = Engine::new()?;
    let mut last = Instant::now();

    'main: loop {
        engine.clear_buffer();
        let events = engine.read_inputs()?;
        let mut i = 10;
        for event in events {
            i += 1;
            match event.code {
                KeyCode::Escape => break 'main,
                code => engine.put_str(10, i, &format!("{:?}", code), Style::default()),
            }
        }

        engine.put_str(
            rand::random::<u8>() as usize,
            rand::random::<u8>() as usize,
            &format!("{}μs", last.elapsed().as_micros()),
            Style::default(),
        );
        last = Instant::now();
        engine.draw()?;
        // std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
