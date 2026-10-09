// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::image::types::DecodeError;

use super::inflate::Sink;
use super::rows::Rows;
use super::scanline::unfilter_row;
use super::to_argb::row_to_argb;

impl Rows<'_> {
    /* A complete scanline: unfilter it, write its pixels at their place in
     * the image (spaced out for an Adam7 pass), keep it as the next row's
     * reference and move on, entering the next pass after the last row. */
    fn finish_row(&mut self) -> Result<(), DecodeError> {
        let bpp = self.h.bits_pp().div_ceil(8).max(1);
        let (filter, row) =
            self.cur[..self.need].split_first_mut().ok_or(DecodeError::Truncated)?;
        unfilter_row(*filter, row, &self.prev[1..self.need], bpp)?;
        let [x0, y0, dx, dy] = self.passes[self.pass];
        let at = (y0 + self.row * dy) * self.h.size.width as usize + x0;
        row_to_argb(self.h, &self.pal, row, self.size.0, self.out, at, dx)?;
        core::mem::swap(&mut self.cur, &mut self.prev);
        (self.fill, self.row) = (0, self.row + 1);
        if self.row == self.size.1 {
            self.enter(self.pass + 1);
        }
        Ok(())
    }
}

impl Sink for Rows<'_> {
    fn put(&mut self, b: u8) -> Result<(), DecodeError> {
        if self.done() {
            return Ok(());
        }
        self.cur[self.fill] = b;
        self.fill += 1;
        if self.fill == self.need {
            self.finish_row()?;
        }
        Ok(())
    }

    fn done(&self) -> bool {
        self.pass >= self.passes.len()
    }
}
