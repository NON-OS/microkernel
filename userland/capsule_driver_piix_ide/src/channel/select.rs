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

//! Selecting a drive before a command.

use super::Channel;
use crate::constants::ata::{dev_select, REG_DEVICE, ST_BSY, ST_DRQ};
use crate::constants::timing::SELECT_MS;
use crate::wait;

/// ATA: 400 ns from writing the device register to a status that is the
/// new drive's. Each read is a trip through the kernel, far longer than
/// 100 ns, so four reads cover it.
const SETTLE_READS: usize = 4;

impl Channel {
    /// Select `drive` and let the status settle on it.
    pub fn select(&self, drive: u8) {
        self.cmd.write8(REG_DEVICE, dev_select(drive));
        for _ in 0..SETTLE_READS {
            let _ = self.alt_status();
        }
    }

    /// Select `drive` and wait until it takes a command: neither busy nor
    /// holding data.
    pub fn ready(&self, drive: u8) -> bool {
        self.select(drive);
        wait::until(SELECT_MS, || self.alt_status() & (ST_BSY | ST_DRQ) == 0)
    }
}
