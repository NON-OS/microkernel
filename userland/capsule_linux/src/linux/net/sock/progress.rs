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

//! How far a waiting call has got. A blocking send on Linux returns once
//! every byte is queued, and a receive with MSG_WAITALL once its buffer is
//! full; a call parked partway keeps its count here, by thread, until it is
//! answered.

use super::cell::with;

pub fn progress(tid: u32) -> usize {
    with(|t| t.progress.iter().find(|p| p.0 == tid).map_or(0, |p| p.1))
}

pub fn set_progress(tid: u32, done: usize) {
    with(|t| {
        t.progress.retain(|p| p.0 != tid);
        if done != 0 {
            t.progress.push((tid, done));
        }
    });
}
