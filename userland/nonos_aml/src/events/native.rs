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

//! Arming an event the capsule serves itself. An event firmware also gave
//! an _Lxx or _Exx is served natively; a wake event is still never armed.

use super::error::EventsError;
use super::io::EventIo;
use super::state::Events;

impl Events {
    pub fn arm_native(
        &mut self,
        io: &mut impl EventIo,
        gpe: u16,
        wake: bool,
    ) -> Result<(), EventsError> {
        if wake {
            return Ok(());
        }
        let b = *self.block_of(gpe).ok_or(EventsError::NoBlock)?;
        if !b.set_enabled(io, gpe, true) {
            return Err(EventsError::Io);
        }
        self.native.insert(gpe);
        self.armed.insert(gpe);
        Ok(())
    }
}
