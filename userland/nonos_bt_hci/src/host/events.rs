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

//! Events for the caller. Number of Completed Packets is the host's own
//! business (it frees ACL buffers) and never reaches the caller; the rest
//! are kept in order while a command is waited for. Each is noted first
//! (`note`), so a controller that lost its state stops the host.

use alloc::vec::Vec;

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;

/// Events kept while a command is waited for; past this the oldest goes,
/// so a flood of advertising reports cannot grow the queue without bound.
pub const QUEUE_MAX: usize = 64;

impl<T: Transport> Host<T> {
    /// Events dropped from the front of a full queue.
    pub fn dropped(&self) -> u32 {
        self.dropped
    }

    pub(super) fn keep(&mut self, event: Vec<u8>) -> Result<(), HciError> {
        if self.note(&event)? {
            return Ok(());
        }
        if self.queue.len() == QUEUE_MAX {
            self.queue.pop_front();
            self.dropped = self.dropped.saturating_add(1);
        }
        self.queue.push_back(event);
        Ok(())
    }

    /// The next event for the caller: a kept one first, then one from the
    /// transport; `None` when nothing for the caller has come. Never waits.
    pub fn next_event(&mut self) -> Result<Option<Vec<u8>>, HciError> {
        if let Some(e) = self.queue.pop_front() {
            return Ok(Some(e));
        }
        match self.t.poll_event()? {
            Some(e) if !self.note(&e)? => Ok(Some(e)),
            _ => Ok(None),
        }
    }
}
