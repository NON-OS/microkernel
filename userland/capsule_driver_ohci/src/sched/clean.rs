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

//! A run is done only when every TD it retired says so. A TD retired with
//! any condition code but NoError halts its ED (OHCI 1.0a 4.3.1.3.5, as
//! recalled, unverified); one a confused controller retired so without the
//! halt is taken as the halt it should have been, never as success.

use super::queue::Ran;
use super::td::cc;
use crate::env::Mem;
use crate::facts::td::{CC_NO_ERROR, TD_LEN};

/// `ran` as it stands once the `n` TDs from `at` are looked at.
pub fn clean<M: Mem>(m: &M, ran: Ran, at: usize, n: usize) -> Ran {
    let errored = (0..n).any(|i| cc(m, at + i * TD_LEN) != CC_NO_ERROR);
    match ran {
        Ran::Done if errored => Ran::Halted,
        other => other,
    }
}
