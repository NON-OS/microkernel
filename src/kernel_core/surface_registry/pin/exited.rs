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

//! Thread groups of guests whose exit has started, so no surface is pinned
//! over their frames after keep_at_exit counted what to keep. A pin that
//! landed between that count and the slot release would be released with
//! no orphan, and its frames freed under a view the compositor could take
//! in between. The mark goes when the group's tables are freed.

use alloc::vec::Vec;

use spin::Mutex;

static EXITED: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// Under the gate, from keep_at_exit: `group`, a guest's, exits.
pub(super) fn mark(group: u32) {
    let mut exited = EXITED.lock();
    if !exited.contains(&group) {
        exited.push(group);
    }
}

/// Under the gate: whether `group`'s exit has started.
pub(super) fn has(group: u32) -> bool {
    EXITED.lock().contains(&group)
}

/// `group`'s tables are freed; its pid may name a new process.
pub fn forget(group: u32) {
    EXITED.lock().retain(|&g| g != group);
}
