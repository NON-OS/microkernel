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

//! Command Completion Events no wait asked for.
//!
//! Commands here are one at a time: `run_command` queues one and waits for
//! it, and gives it up with an abort if it never answers. An abort, or a
//! command that answers just after its wait gave up, leaves completions on
//! the ring that no wait will ever take: Command Aborted, Command Ring
//! Stopped, the late answer. The drain and the interrupt-IN scan both
//! stopped at the first of them, so a keyboard went quiet until the next
//! command happened to take it off. Outside a command wait, every such
//! completion is an orphan, and is taken off and counted.

use super::state::EventRing;
use crate::constants::TRB_TYPE_CMD_COMPLETION_EVENT;

impl EventRing {
    /// Take the event at the dequeue pointer off if it is a command
    /// completion, which outside a command wait no one waits for. The caller
    /// writes ERDP afterwards, as for any event it consumes.
    pub fn skip_orphan(&mut self) -> bool {
        if !self.has_event() || self.current_trb().get_type() != TRB_TYPE_CMD_COMPLETION_EVENT {
            return false;
        }
        self.advance();
        self.orphans += 1;
        true
    }

    /// Orphan command completions taken off since the start.
    pub fn orphans(&self) -> u64 {
        self.orphans
    }
    /// Events given up to make room, never one an armed transfer awaits.
    pub fn parked_given_up(&self) -> u64 {
        self.parked.given_up
    }
}
