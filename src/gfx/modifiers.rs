#[derive(PartialEq, Clone, Copy)]
pub struct Modifiers(u8);

const ENABLE: [u8; 8] = [1, 2, 3, 4, 5, 7, 8, 9];
const DISABLE: [u8; 8] = [22, 22, 23, 24, 25, 27, 28, 29];

impl Modifiers {
    pub const NONE: Self = Self(0);
    pub const BOLD: Self = Self(1);
    pub const DIM: Self = Self(1 << 1);
    pub const ITALIC: Self = Self(1 << 2);
    pub const UNDERLINE: Self = Self(1 << 3);
    pub const BLINKING: Self = Self(1 << 4);
    pub const INVERSE: Self = Self(1 << 5);
    pub const HIDDEN: Self = Self(1 << 6);
    pub const STRIKETHROUGH: Self = Self(1 << 7);

    pub fn add_modifier(&mut self, modifier: Self) {
        self.0 |= modifier.0;
    }

    pub fn remove_modifier(&mut self, modifier: Self) {
        self.0 &= !modifier.0;
    }

    pub fn get_code_change(&self, old: Self) -> String {
        let mut out = Vec::new();

        let xor = self.0 ^ old.0;
        let add = xor & self.0;
        let remove = xor & old.0;

        for bit in 0..8 {
            let mask = 1 << bit;
            if add & mask > 0 {
                out.push(ENABLE[bit].to_string());
            }
            if remove & mask > 0 {
                out.push(DISABLE[bit].to_string());
            }
        }

        out.join(";")
    }
}
