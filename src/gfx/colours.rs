#[derive(PartialEq)]
pub enum Colour {
    Rgb(u8, u8, u8), // Truecolour r,g,b
    C256(u8),        // 256 colour mode
    C16(u8),         // 16 colour mode
    None,
}

impl Colour {
    pub fn get_code(&self, foreground: bool) -> String {
        match self {
            Colour::Rgb(r, g, b) => {
                format!("{};2;{};{};{}", if foreground { 38 } else { 48 }, r, g, b)
            }
            Colour::C256(n) => {
                format!("{};5;{}", if foreground { 38 } else { 48 }, n)
            }
            Colour::C16(n) => {
                format!("{}", n + if foreground { 0 } else { 10 })
            }
            _ => String::new(),
        }
    }
}
