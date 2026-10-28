use crate::gfx::{colours::Colour, modifiers::Modifiers};

#[derive(Clone, Copy)]
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

        out.push(self.modifiers.get_code_change(old.modifiers));

        out.join(";")
    }
}
