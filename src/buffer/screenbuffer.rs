use super::cell::Cell;
use crate::gfx::style::Style;

pub(crate) struct ScreenBuffer {
    cells: Vec<Cell>,

    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: vec![Cell::default(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn write_cell(&mut self, x: usize, y: usize, c: char, style: Style) {
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        let index = x + y * self.width as usize;
        self.cells[index].char = c;
        self.cells[index].style = style;
    }

    pub(crate) fn into_iter(&self) -> ScreenBufferIter<'_> {
        ScreenBufferIter {
            buff: self,
            index: 0,
        }
    }

    pub(crate) fn clear_buffer(&mut self) {
        self.cells = vec![Cell::default(); (self.width * self.height) as usize];
    }
}

pub(crate) struct ScreenBufferIter<'a> {
    buff: &'a ScreenBuffer,
    index: usize,
}

impl Iterator for ScreenBufferIter<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buff.cells.len() {
            return None;
        }

        let cell = self.buff.cells[self.index];

        self.index += 1;

        Some(cell)
    }
}
