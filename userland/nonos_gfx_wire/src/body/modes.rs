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

//! The MODES body, encoded and checked.

use super::mode::{Mode, MAX_MODES, MODE_LEN};
use crate::bytes::{put, u32_at};
use crate::errno::{Errno, EINVAL};

/// Up to 20 modes, held without allocating.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modes {
    pub list: [Mode; MAX_MODES],
    pub len: usize,
}

impl Modes {
    pub fn as_slice(&self) -> &[Mode] {
        &self.list[..self.len]
    }

    /// The body, into `out`; the bytes written.
    pub fn encode(&self, out: &mut [u8]) -> Result<usize, Errno> {
        let n = self.len.min(MAX_MODES);
        let mut ok = put(out, 0, &(n as u32).to_le_bytes());
        for (i, m) in self.list[..n].iter().enumerate() {
            let at = 4 + i * MODE_LEN;
            ok &= put(out, at, &m.width.to_le_bytes())
                && put(out, at + 4, &m.height.to_le_bytes())
                && put(out, at + 8, &m.flags.to_le_bytes());
        }
        if ok {
            Ok(4 + n * MODE_LEN)
        } else {
            Err(EINVAL)
        }
    }

    /// Read a MODES body; a count past 20 or past the body is refused.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let n = u32_at(b, 0).ok_or(EINVAL)? as usize;
        if n > MAX_MODES || b.len() < 4 + n * MODE_LEN {
            return Err(EINVAL);
        }
        let mut m = Modes { len: n, ..Modes::default() };
        for (i, slot) in m.list[..n].iter_mut().enumerate() {
            let at = 4 + i * MODE_LEN;
            let f = |o| u32_at(b, at + o).ok_or(EINVAL);
            *slot = Mode { width: f(0)?, height: f(4)?, flags: f(8)? };
        }
        Ok(m)
    }
}
