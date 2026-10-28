use std::io;

use rustui::terminal::Terminal;

fn main() -> io::Result<()> {
    let term = Terminal::setup_raw()?;

    loop {
        println!(":3");
    }

    Ok(())
}
