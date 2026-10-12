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

//! Which armed GPEs are asserting: status and enable both set. An event
//! that is set but not armed is left alone; it raises no SCI while disabled.

use super::error::EventsError;
use super::io::EventIo;
use super::state::Events;
use alloc::vec::Vec;

impl Events {
    pub(super) fn pending(&self, io: &mut impl EventIo) -> Result<Vec<u16>, EventsError> {
        let mut out = Vec::new();
        for b in &self.blocks {
            for i in 0..u16::from(b.half) {
                let sts = io.read(b.port + i, 1).ok_or(EventsError::Io)?;
                let en = io.read(b.port + u16::from(b.half) + i, 1).ok_or(EventsError::Io)?;
                let hit = sts & en;
                for bit in 0..8 {
                    let n = b.base + i * 8 + bit;
                    if hit & (1 << bit) != 0 && self.armed.contains(&n) {
                        out.push(n);
                    }
                }
            }
        }
        Ok(out)
    }
}
