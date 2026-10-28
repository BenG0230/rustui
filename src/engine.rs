use std::io;

use crate::{
    buffer::screenbuffer::ScreenBuffer,
    gfx::style::Style,
    input::{KeyEvent, read_input},
    renderer::draw,
    terminal::Terminal,
};

pub struct Engine {
    term: Terminal,
    buff: ScreenBuffer,
}

impl Engine {
    pub fn new() -> io::Result<Self> {
        let term = Terminal::setup_raw()?;
        let buff = ScreenBuffer::new(term.width, term.height);

        Ok(Self { term, buff })
    }

    pub fn put_char(&mut self, x: usize, y: usize, c: char, style: Style) {
        self.buff.write_cell(x, y, c, style);
    }

    pub fn put_str(&mut self, mut x: usize, y: usize, s: &str, style: Style) {
        for char in s.chars() {
            self.put_char(x, y, char, style);
            x += 1;
        }
    }

    pub fn read_inputs(&self) -> io::Result<Vec<KeyEvent>> {
        read_input()
    }

    pub fn clear_buffer(&mut self) {
        self.buff.clear_buffer();
    }

    pub fn draw(&self) -> io::Result<()> {
        draw(&self.buff)
    }
}
