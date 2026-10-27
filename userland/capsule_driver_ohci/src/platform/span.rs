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
//! The bound every access to the schedule region is held to. An access
//! past it is refused, not made, and the refusal kept until asked for, so
//! the request that made it is answered as failed (server/dispatch.rs)
//! rather than the capsule aborting on an assertion.

use crate::env::OutOfRegion;
use core::cell::Cell;

pub struct Span {
    len: usize,
    faulted: Cell<bool>,
}

impl Span {
    pub const fn new(len: usize) -> Self {
        Span { len, faulted: Cell::new(false) }
    }

    /// `off`, when `off..off + len` lies inside; else the fault, kept.
    pub fn check(&self, off: usize, len: usize) -> Result<usize, OutOfRegion> {
        if off.checked_add(len).is_some_and(|end| end <= self.len) {
            return Ok(off);
        }
        self.faulted.set(true);
        Err(OutOfRegion)
    }

    /// The fault kept since the last asking, then forgotten.
    pub fn fault(&self) -> Result<(), OutOfRegion> {
        match self.faulted.replace(false) {
            true => Err(OutOfRegion),
            false => Ok(()),
        }
    }
}
