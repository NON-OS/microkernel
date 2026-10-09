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

use alloc::vec;
use alloc::vec::Vec;

use super::adam7::{next_pass, ADAM7, PLAIN};
use super::header::Header;
use super::palette::Palette;

/* Scanline assembly: inflated bytes fill the current row; a full row is
 * unfiltered against the previous one and written straight into the output,
 * so only two rows exist at a time. Adam7 walks its seven passes in turn. */
pub(super) struct Rows<'a> {
    pub(super) h: &'a Header,
    pub(super) pal: Palette<'a>,
    pub(super) out: &'a mut [u32],
    pub(super) passes: &'static [[usize; 4]],
    pub(super) cur: Vec<u8>,
    pub(super) prev: Vec<u8>,
    pub(super) fill: usize,
    pub(super) need: usize,
    pub(super) pass: usize,
    pub(super) row: usize,
    pub(super) size: (usize, usize),
}

impl<'a> Rows<'a> {
    pub fn new(h: &'a Header, pal: Palette<'a>, out: &'a mut [u32]) -> Self {
        let full = h.row_bytes(h.size.width as usize) + 1;
        let passes: &'static [[usize; 4]] = if h.interlaced { &ADAM7 } else { &PLAIN };
        let (cur, prev) = (vec![0u8; full], vec![0u8; full]);
        let mut r = Self {
            h,
            pal,
            out,
            passes,
            cur,
            prev,
            fill: 0,
            need: 0,
            pass: 0,
            row: 0,
            size: (0, 0),
        };
        r.enter(0);
        r
    }

    /* Start the first pass at or after `p` that holds any pixels. */
    pub(super) fn enter(&mut self, p: usize) {
        (self.pass, self.size) = next_pass(self.passes, p, self.h.size);
        self.need = self.h.row_bytes(self.size.0) + 1;
        self.row = 0;
        self.prev.fill(0);
    }
}
