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

//! What a halted run's failed TD said, and the errors a transfer answers.

use crate::env::Mem;
use crate::facts::td::{CC_NOT_ACCESSED, CC_NO_ERROR, CC_STALL, TD_LEN};
use crate::sched::td::cc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CtrlError {
    Stalled,
    /// Another condition code (facts::td).
    Bus(u32),
    TimedOut,
    /// Longer than the data buffer.
    TooLong,
}

/// The first of the `n` TDs from `base` the controller retired with an
/// error.
pub fn failed<M: Mem>(m: &M, base: usize, n: usize) -> CtrlError {
    let bad = (0..n).map(|i| cc(m, base + i * TD_LEN));
    match bad.into_iter().find(|&c| c != CC_NO_ERROR && c != CC_NOT_ACCESSED) {
        Some(CC_STALL) => CtrlError::Stalled,
        Some(c) => CtrlError::Bus(c),
        None => CtrlError::Bus(0),
    }
}
