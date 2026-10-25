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

//! An EC that stops answering or will not stop asking. Three waits in a row
//! that time out mean it has stalled; more events than the storm bound in a
//! window mean it floods. Either way the caller turns it off: its space is
//! refused and its GPE quieted, so one bad EC cannot hold the capsule.
//! Three is our bound, from no source.

use super::regs::EcError;

pub const STRIKES: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Off {
    Stalled,
    Flooded,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Health {
    strikes: u8,
    pub off: Option<Off>,
}

impl Health {
    pub fn usable(&self) -> bool {
        self.off.is_none()
    }

    /// After one access: a timeout is a strike and any answer clears them;
    /// a port the kernel refused is neither. True when this turned it off.
    pub fn note<T>(&mut self, r: &Result<T, EcError>) -> bool {
        match r {
            Err(EcError::Timeout) => self.strikes = self.strikes.saturating_add(1),
            Ok(_) => self.strikes = 0,
            Err(EcError::Io) => {}
        }
        self.turn_off(self.strikes >= STRIKES, Off::Stalled)
    }

    /// The EC raised past the storm bound. True when this turned it off.
    pub fn flooded(&mut self) -> bool {
        self.turn_off(true, Off::Flooded)
    }

    fn turn_off(&mut self, now: bool, why: Off) -> bool {
        let newly = now && self.off.is_none();
        if newly {
            self.off = Some(why);
        }
        newly
    }
}
