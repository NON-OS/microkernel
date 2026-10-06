use crate::image::types::DecodeError;

use super::frame::Frame;

/* Where decoded palette indices land: stream position (x, stream row) maps in
 * constant time to a screen pixel; off-screen columns and rows are dropped,
 * transparent indices skipped. Done once every visible row is in. */
pub(super) struct Canvas<'a> {
    out: &'a mut [u32],
    palette: &'a [u8],
    transparent: Option<u8>,
    f: &'a Frame,
    screen: (usize, usize),
    x: usize,
    r: usize,
    dst: Option<usize>,
    rows_left: usize,
}

impl<'a> Canvas<'a> {
    pub fn new(
        out: &'a mut [u32],
        palette: &'a [u8],
        transparent: Option<u8>,
        f: &'a Frame,
        screen: (usize, usize),
        visible: usize,
    ) -> Self {
        let mut c = Self {
            out,
            palette,
            transparent,
            f,
            screen,
            x: 0,
            r: 0,
            dst: None,
            rows_left: visible,
        };
        c.dst = c.row_base(0);
        c
    }

    /* Screen offset of stream row `r`'s first pixel, None when off screen. */
    fn row_base(&self, r: usize) -> Option<usize> {
        let y = self.f.top + self.f.row(r)?;
        (y < self.screen.1).then_some(y * self.screen.0)
    }

    pub fn put(&mut self, idx: u8) -> Result<(), DecodeError> {
        let dx = self.f.left + self.x;
        if let Some(base) = self.dst {
            if dx < self.screen.0 && self.transparent != Some(idx) {
                let e = idx as usize * 3;
                let rgb = self.palette.get(e..e + 3).ok_or(DecodeError::Unsupported)?;
                let px = self.out.get_mut(base + dx).ok_or(DecodeError::OutputTooSmall)?;
                *px = 0xff00_0000 | (rgb[0] as u32) << 16 | (rgb[1] as u32) << 8 | rgb[2] as u32;
            }
        }
        self.x += 1;
        if self.x == self.f.w {
            self.x = 0;
            if self.dst.is_some() {
                self.rows_left = self.rows_left.saturating_sub(1);
            }
            self.r += 1;
            self.dst = self.row_base(self.r);
        }
        Ok(())
    }

    pub fn done(&self) -> bool {
        self.rows_left == 0
    }
}
