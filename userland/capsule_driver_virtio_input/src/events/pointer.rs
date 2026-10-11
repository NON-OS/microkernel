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

//! A pointer report: an absolute position, relative motion, a wheel step
//! and button changes. Gathering is in gather.rs, posting in post_pointer.rs.

/// More button changes than any report carries; past it they are dropped.
pub const MAX_BUTTONS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pointer {
    /// The position in the absolute range, and whether this report moved it.
    pub abs: (i32, i32),
    pub abs_moved: bool,
    pub rel: (i32, i32),
    pub wheel: i32,
    pub buttons: [(u32, bool); MAX_BUTTONS],
    pub n_buttons: usize,
}

impl Pointer {
    pub const fn empty() -> Self {
        Self {
            abs: (0, 0),
            abs_moved: false,
            rel: (0, 0),
            wheel: 0,
            buttons: [(0, false); MAX_BUTTONS],
            n_buttons: 0,
        }
    }

    pub const fn after(done: &Self) -> Self {
        let mut p = Self::empty();
        p.abs = done.abs;
        p
    }
}
