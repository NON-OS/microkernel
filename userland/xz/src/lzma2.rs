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

//! LZMA2: chunks that are stored, or LZMA with a declared reset of the
//! dictionary, the state, or the properties, ended by a zero control byte.

use alloc::vec::Vec;

use super::limits::MAX_OUT;
use super::lzma2_chunk::{lzma, State};

/// Decode into `out`; the bytes the LZMA2 data took.
pub fn lzma2(d: &[u8], dict: u32, out: &mut Vec<u8>) -> Option<usize> {
    let mut st = State { start: out.len(), fresh: false, model: None, dict };
    let mut at = 0usize;
    loop {
        let control = *d.get(at)?;
        at = match control {
            0x00 => return Some(at + 1),
            0x01 | 0x02 => {
                if control == 0x01 {
                    (st.start, st.fresh) = (out.len(), true);
                }
                let size =
                    usize::from(u16::from_be_bytes(d.get(at + 1..at + 3)?.try_into().ok()?)) + 1;
                out.extend_from_slice(d.get(at + 3..at + 3 + size)?);
                at + 3 + size
            }
            0x80..=0xFF => lzma(d, at, &mut st, out)?,
            _ => return None,
        };
        // The first chunk must reset the dictionary; nothing before it is history.
        if !st.fresh || out.len() > MAX_OUT {
            return None;
        }
    }
}
