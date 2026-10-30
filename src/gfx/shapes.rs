use crate::{buffer::screenbuffer::ScreenBuffer, gfx::style::Style};

pub(crate) fn draw_vline(
    buff: &mut ScreenBuffer,
    x: usize,
    starty: usize,
    endy: usize,
    c: char,
    style: Style,
) {
    for y in starty..=endy {
        buff.write_cell(x, y, c, style);
    }
}

pub(crate) fn draw_hline(
    buff: &mut ScreenBuffer,
    y: usize,
    startx: usize,
    endx: usize,
    c: char,
    style: Style,
) {
    for x in startx..=endx {
        buff.write_cell(x, y, c, style);
    }
}

pub(crate) fn draw_line(
    buff: &mut ScreenBuffer,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    c: char,
    style: Style,
) {
    let dx: i32 = x1 as i32 - x0 as i32;
    let dy: i32 = y1 as i32 - y0 as i32;

    let mut d: i32 = (2 * (dy - dx)) as i32;
    let mut y = y0;

    for x in x0..=x1 {
        buff.write_cell(x, y, c, style);
        print!("{d}");
        if d > 0 {
            y += 1;
            d += (2 * (dy - dx)) as i32;
        } else {
            d += (2 * dy) as i32;
        }
    }
}

pub(crate) fn draw_square(
    buff: &mut ScreenBuffer,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    c: char,
    style: Style,
) {
    draw_hline(buff, y0, x0, x1 - 1, c, style);
    draw_hline(buff, y1, x0 + 1, x1, c, style);
    draw_vline(buff, x0, y0 + 1, y1, c, style);
    draw_vline(buff, x1, y0, y1 - 1, c, style);
}
