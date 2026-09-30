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

//! Getting a BOT device back in step: a stalled pipe is cleared on both
//! sides (USB MSC BOT 1.0, section 6.6.1), and a lost phase is answered
//! with the Bulk-Only Mass Storage Reset and both pipes cleared (5.3.4).

use super::types::Disk;
use crate::xhci::{control_no_data, reset_bulk};

const CLEAR_FEATURE: (u8, u8) = (0x02, 0x01);
const MASS_STORAGE_RESET: (u8, u8) = (0x21, 0xFF);

/// Clear a halted pipe in the controller, then the device's halt feature.
pub fn clear_halt(disk: &Disk, dir_in: bool) -> Result<(), i32> {
    reset_bulk(disk.xhci, disk.slot, dir_in)?;
    let ep = if dir_in { disk.ep_in } else { disk.ep_out };
    control_no_data(disk.xhci, disk.slot, CLEAR_FEATURE, 0, ep as u16)
}

/// Reset the device's BOT state machine. A pipe that did not halt makes
/// its controller-side reset fail; the device-side clear still runs.
pub fn reset_recovery(disk: &Disk) -> Result<(), i32> {
    control_no_data(disk.xhci, disk.slot, MASS_STORAGE_RESET, 0, disk.interface as u16)?;
    for (dir_in, ep) in [(true, disk.ep_in), (false, disk.ep_out)] {
        let _ = reset_bulk(disk.xhci, disk.slot, dir_in);
        control_no_data(disk.xhci, disk.slot, CLEAR_FEATURE, 0, ep as u16)?;
    }
    Ok(())
}
