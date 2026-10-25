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

//! Bringing the events up: everything off and cleared first, so nothing
//! firmware left pending fires into a handler that is not ready, then the
//! chosen fixed events and the armed runtime events on.

use super::error::EventsError;
use super::gpe_block::GpeBlock;
use super::io::EventIo;
use super::pm1::{GBL, PWRBTN, RTC, SLPBTN, TMR, WAK};
use super::state::Events;

/// The PM1 status bits cleared at start: every one the capsule serves or
/// never enables. A button it leaves to the kernel keeps its pending press.
const fn stale(fixed: u16) -> u16 {
    TMR | GBL | RTC | WAK | (fixed & (PWRBTN | SLPBTN))
}

impl Events {
    pub fn start(&self, io: &mut impl EventIo) -> Result<(), EventsError> {
        let ok = self.blocks.iter().all(|b| quiet(b, io))
            && self.pm1.set_enable(io, 0)
            && self.pm1.clear(io, stale(self.fixed))
            && self.pm1.set_enable(io, self.fixed);
        if !ok {
            return Err(EventsError::Io);
        }
        for &n in &self.armed {
            let b = self.block_of(n).ok_or(EventsError::Io)?;
            if !b.set_enabled(io, n, true) {
                return Err(EventsError::Io);
            }
        }
        Ok(())
    }
}

/// Every enable off and every status cleared, as the capsule starts.
fn quiet(b: &GpeBlock, io: &mut impl EventIo) -> bool {
    (0..u16::from(b.half))
        .all(|i| io.write(b.port + u16::from(b.half) + i, 1, 0) && io.write(b.port + i, 1, 0xFF))
}
