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

//! A wake event armed while the system runs (runtime wake, ACPI 6.5,
//! 7.3.13 and 5.6.6): enabled and served like any runtime event, through
//! firmware's _Lxx or _Exx, whose Notify(device, 2) says the device asked
//! to be woken. Only an event firmware gave a method is armed; one with
//! none would fire with nothing to quiet its source.

use super::error::EventsError;
use super::io::EventIo;
use super::state::Events;

impl Events {
    pub fn arm_wake(&mut self, io: &mut impl EventIo, gpe: u16, on: bool) -> Result<(), EventsError> {
        let b = *self.block_of(gpe).ok_or(EventsError::NoBlock)?;
        if on && !self.handlers.contains_key(&gpe) {
            return Err(EventsError::NoMethod);
        }
        if !b.set_enabled(io, gpe, on) {
            return Err(EventsError::Io);
        }
        if on {
            self.armed.insert(gpe);
        } else {
            self.armed.remove(&gpe);
        }
        Ok(())
    }
}
