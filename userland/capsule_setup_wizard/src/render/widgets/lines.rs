use nonos_toolkit::font::render::draw_text;

/// Plain text, one line every 20 pixels from `y`. Returns the y below it.
pub fn text(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    lines: &[&[u8]],
    color: u32,
) -> u32 {
    let mut yy = y;
    for line in lines {
        draw_text(buf, spx, w, h, x, yy, line, color);
        yy += 20;
    }
    yy
}
