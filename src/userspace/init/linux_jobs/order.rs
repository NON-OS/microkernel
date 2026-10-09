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

//! The order queued jobs run in when some have to wait. Pure, so the kernel
//! proofs hold it.

use alloc::vec::Vec;

/// What had to wait, ahead of what was asked meanwhile, each once: installs
/// run in the order they were asked for, however long the one before took.
pub(crate) fn requeue<T: PartialEq>(waiting: Vec<T>, newer: Vec<T>) -> Vec<T> {
    let mut out = waiting;
    for job in newer {
        if !out.contains(&job) {
            out.push(job);
        }
    }
    out
}
