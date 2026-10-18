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

//! The rings the waits run over, and the entries the controller writes.

use crate::admin::{Completion, CqCursor};

/// Ring sizes: one and two entries for the wrap edges, the I/O queue's 8 and
/// the admin queue's 64.
pub(crate) const RINGS: [u16; 4] = [1, 2, 8, 64];

pub(crate) fn entry(cid: u16, sq_id: u16, sq_head: u16, status: u16) -> Completion {
    Completion { dw0: 0, dw1: 0, sq_head, sq_id, cid, status }
}

/// A status word with the given phase tag and nothing else set: success.
pub(crate) fn ok_status(phase: bool) -> u16 {
    phase as u16
}

/// A cursor at slot `head` of an `entries` ring, waiting for `phase`.
pub(super) fn cursor_at(entries: u16, head: u16, phase: bool) -> CqCursor {
    let mut c = CqCursor::new(entries);
    c.head = head;
    c.phase = phase;
    c
}
