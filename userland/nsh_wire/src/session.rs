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

//! The terminal bodies: the bytes of `INPUT` and `OUTPUT`, and the window
//! size. Each is read whole and exactly.

use crate::limits::MAX_DATA;
use crate::{Reader, WireError, Writer};

/// The largest window either way, in cells.
pub const MAX_CELLS: u16 = 1000;

/// The bytes of an `INPUT` or `OUTPUT`: 1 to `MAX_DATA` of them.
pub fn data(body: &[u8]) -> Result<&[u8], WireError> {
    match body.len() {
        0 => Err(WireError::Value),
        n if n > MAX_DATA => Err(WireError::Oversize),
        _ => Ok(body),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub cols: u16,
    pub rows: u16,
}

pub fn window(body: &[u8]) -> Result<Window, WireError> {
    let mut r = Reader::new(body);
    let w = Window { cols: r.u16()?, rows: r.u16()? };
    r.finish()?;
    w.check().map(|_| w)
}

impl Window {
    fn check(&self) -> Result<(), WireError> {
        let ok = |n: u16| (1..=MAX_CELLS).contains(&n);
        match ok(self.cols) && ok(self.rows) {
            true => Ok(()),
            false => Err(WireError::Value),
        }
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<usize, WireError> {
        self.check()?;
        let mut w = Writer::new(out);
        w.u16(self.cols)?;
        w.u16(self.rows)?;
        Ok(w.done())
    }
}
