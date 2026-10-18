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

//! What an SCI does first and what it does to an event past its bound: the
//! fixed events read and cleared before any GPE method runs, so a pressed
//! power button is seen however long the methods take; and a stormed or
//! dead event turned off for good, its status cleared.

use super::error::EventsError;
use super::gpe_block::GpeBlock;
use super::io::EventIo;
use super::state::Events;

impl Events {
    pub(super) fn block_of(&self, gpe: u16) -> Option<&GpeBlock> {
        self.blocks.iter().find(|b| b.holds(gpe))
    }

    /// The fixed events asserting now, each one this capsule serves cleared.
    pub fn take_fixed(&mut self, io: &mut impl EventIo) -> Result<u16, EventsError> {
        let fixed = self.pm1.status(io).ok_or(EventsError::Io)? & self.fixed;
        if fixed != 0 && !self.pm1.clear(io, fixed) {
            return Err(EventsError::Io);
        }
        Ok(fixed)
    }

    /// Turns GPE `n` off: disabled, its status cleared, no longer armed.
    pub fn quiet(&mut self, io: &mut impl EventIo, n: u16) -> Result<(), EventsError> {
        self.armed.remove(&n);
        let Some(b) = self.block_of(n).copied() else { return Ok(()) };
        if b.set_enabled(io, n, false) && b.clear(io, n) {
            Ok(())
        } else {
            Err(EventsError::Io)
        }
    }
}
