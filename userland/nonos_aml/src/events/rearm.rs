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

//! Arming again, from the capsule's loop, each event whose back-off window
//! has passed (`storm_backoff`). Its status is left as it stands, so an
//! event latched while it was off, or a level source still asserting, is
//! served at the next SCI rather than lost.

use super::error::EventsError;
use super::io::EventIo;
use super::state::Events;
use alloc::vec::Vec;

impl Events {
    pub fn rearm_due(&mut self, io: &mut impl EventIo) -> Result<Vec<u16>, EventsError> {
        let now = io.now_ms();
        let due = self.storms.due(now);
        for n in &due {
            let Some(b) = self.block_of(*n).copied() else { continue };
            b.set_enabled(io, *n, true).then_some(()).ok_or(EventsError::Io)?;
            self.armed.insert(*n);
        }
        Ok(due)
    }
}
