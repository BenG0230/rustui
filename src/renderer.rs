use std::io::{self, Write};

use crate::{buffer::screenbuffer::ScreenBuffer, gfx::style::Style};

pub fn display(buff: &ScreenBuffer) -> io::Result<()> {
    let mut buff_iter = buff.into_iter();

    let mut out = String::new();
    let mut current_style = Style::default();

    while let Some(cell) = buff_iter.next() {
        if cell.style != current_style {
            let style_string = cell.style.get_new_code(current_style);

            if !style_string.is_empty() {
                out.push_str("\x1b[");
                out.push_str(&style_string);
                out.push('m');
            }
        }

        out.push(cell.char);

        current_style = cell.style;
    }
    print!("\x1b[H{}", out);
    io::stdout().flush()?;
    Ok(())
}
