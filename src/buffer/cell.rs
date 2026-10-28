use crate::gfx::{colours::Colour, modifiers::Modifiers, style::Style};

#[derive(Clone, Copy)]
pub(crate) struct Cell {
    pub char: char,
    pub style: Style,
}

impl Cell {
    pub const fn default() -> Self {
        Self {
            char: ' ',
            style: Style {
                fg: Colour::None,
                bg: Colour::None,
                modifiers: Modifiers::NONE,
            },
        }
    }
}
