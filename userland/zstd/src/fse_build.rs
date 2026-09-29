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

//! Building an FSE decoding table from normalized counts (RFC 8878 4.1.1).

use alloc::vec;
use alloc::vec::Vec;

pub use super::fse_cell::{Cell, Fse};

pub fn build(log: u8, norm: &[i16]) -> Option<Fse> {
    let size = 1usize << log;
    let mut cells = vec![Cell::default(); size];
    let mut next: Vec<u32> = vec![0; norm.len()];
    // The last cell not yet given to a "less than one" symbol.
    let mut high = size as isize - 1;
    for (s, &n) in norm.iter().enumerate() {
        if n == -1 {
            cells.get_mut(usize::try_from(high).ok()?)?.sym = s as u8;
            high -= 1;
            next[s] = 1;
        } else {
            next[s] = n.max(0) as u32;
        }
    }
    if high < 0 && norm.iter().any(|&n| n > 0) {
        return None;
    }
    let step = (size >> 1) + (size >> 3) + 3;
    let mut pos = 0usize;
    for (s, &n) in norm.iter().enumerate() {
        for _ in 0..n.max(0) {
            cells[pos].sym = s as u8;
            pos = (pos + step) & (size - 1);
            while pos as isize > high {
                pos = (pos + step) & (size - 1);
            }
        }
    }
    if pos != 0 {
        return None;
    }
    for c in cells.iter_mut() {
        let state = next.get_mut(c.sym as usize)?;
        let top = 31u32.checked_sub(state.leading_zeros())?;
        let bits = u32::from(log).checked_sub(top)?;
        c.bits = bits as u8;
        c.base = ((*state << bits) as usize).checked_sub(size)? as u16;
        *state += 1;
    }
    Some(Fse { log, cells })
}
