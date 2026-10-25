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

//! What a test reads back of the mappings, and the cue that refuses one.

use std::cell::Cell;

use crate::mem::{MAPS, REFUSE, WRONG};

/// Each live mapping, as (base, length).
pub fn maps() -> Vec<(usize, usize)> {
    MAPS.with(|m| m.borrow().iter().map(|(b, p)| (*b, p.len())).collect())
}

pub fn refuse_maps(on: bool) {
    REFUSE.with(|r| r.set(on));
}

pub fn unmapped_wrong() -> u32 {
    WRONG.with(Cell::get)
}

/// The bytes of the live mapping holding `[at, at + len)`, if one does.
pub(crate) fn bytes_at(at: usize, len: usize) -> Option<Vec<u8>> {
    MAPS.with(|m| {
        let m = m.borrow();
        let (base, p) = m.range(..=at).next_back()?;
        let from = at - base;
        p.get(from..from.checked_add(len)?).map(<[u8]>::to_vec)
    })
}
