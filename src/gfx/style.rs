use crate::gfx::{colours::Colour, modifiers::Modifiers};

#[derive(Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Colour,
    pub bg: Colour,
    pub modifiers: Modifiers,
}

impl Style {
    pub const fn default() -> Self {
        Self {
            fg: Colour::None,
            bg: Colour::None,
            modifiers: Modifiers::NONE,
        }
    }

    pub fn get_new_code(&self, old: Self) -> String {
        let mut out = Vec::new();

        if self.fg != old.fg {
            out.push(self.fg.get_code(true));
        }

        if self.bg != old.bg {
            out.push(self.bg.get_code(false));
        }

        let modifiers = self.modifiers.get_code_change(old.modifiers);
        if !modifiers.is_empty() {
            out.push(modifiers);
        }

        out.join(";")
    }
}
