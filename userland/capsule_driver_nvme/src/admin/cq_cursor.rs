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

/// Where the driver reads its next completion: the head slot of a ring of
/// `entries`, and the phase tag the controller writes there on this pass.
/// Only the driver consuming an entry moves it; nothing the controller
/// reports (the SQ head in each entry among it) is ever folded in.
#[derive(Clone, Copy)]
pub struct CqCursor {
    pub head: u16,
    pub phase: bool,
    pub entries: u16,
}

impl CqCursor {
    /// A new queue: slot 0, where the controller's first pass writes phase 1.
    pub const fn new(entries: u16) -> Self {
        Self { head: 0, phase: true, entries }
    }

    /// Step past the slot just consumed, flipping the phase on the wrap.
    pub fn advance(&mut self) {
        let next = self.head.wrapping_add(1);
        if next >= self.entries {
            self.head = 0;
            self.phase = !self.phase;
        } else {
            self.head = next;
        }
    }
}
