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

//! Which refused regions have been told: each (space, region base) once,
//! the first `BOUND` of them, then one line saying the rest go untold, so
//! AML retrying a refused field cannot fill the log. Every refusal is
//! counted all the same. Pure.

use alloc::vec::Vec;

pub const BOUND: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tell {
    New,
    /// The bound reached: this one is told, and said to be the last.
    Last,
    Quiet,
}

pub fn tell(told: &mut Vec<(u8, u64)>, key: (u8, u64)) -> Tell {
    let verdict = match (told.contains(&key), told.len().cmp(&BOUND)) {
        (true, _) | (false, core::cmp::Ordering::Greater) => Tell::Quiet,
        (false, core::cmp::Ordering::Less) => Tell::New,
        (false, core::cmp::Ordering::Equal) => Tell::Last,
    };
    if verdict != Tell::Quiet {
        told.push(key);
    }
    verdict
}
