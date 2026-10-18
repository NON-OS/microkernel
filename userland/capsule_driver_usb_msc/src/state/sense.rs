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

//! Autosense: a UAS device sends a command's sense data with its CHECK
//! CONDITION (its Sense IU), where a BOT device keeps it for REQUEST SENSE.
//! The driver asks REQUEST SENSE after either; for UAS it is answered from
//! what was kept, and the device is not asked.

use super::types::State;

/// Fixed-format sense: as much as the driver ever asks for.
pub const SENSE_KEPT: usize = 18;

impl State {
    pub fn keep_sense(&mut self, sense: &[u8]) {
        let mut kept = [0u8; SENSE_KEPT];
        let n = sense.len().min(SENSE_KEPT);
        kept[..n].copy_from_slice(&sense[..n]);
        self.sense = Some((kept, n));
    }

    /// The sense kept since the last CHECK CONDITION, once.
    pub fn take_sense(&mut self) -> Option<([u8; SENSE_KEPT], usize)> {
        self.sense.take()
    }
}
