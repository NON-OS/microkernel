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
use crate::protocol::{E_IO, E_TIMEDOUT};
use crate::state::{Said, State};
use crate::xhci::{control_no_data, reset_bulk, Window};

const CLEAR_FEATURE: (u8, u8) = (0x02, 0x01);
const MASS_STORAGE_RESET: (u8, u8) = (0x21, 0xFF);
/// A reset's own time, whatever the command it follows had left: long
/// enough for driver.xhci0 to finish a transfer that call stopped waiting
/// on, then the five requests.
const RECOVERY_MS: u64 = 8_000;

/// Clear a halted pipe in the controller, then the device's halt feature.
pub fn clear_halt(disk: &Disk, dir_in: bool) -> Result<(), i32> {
    reset_bulk(disk.xhci, disk.slot, dir_in)?;
    let ep = if dir_in { disk.ep_in } else { disk.ep_out };
    control_no_data(disk.xhci, disk.slot, CLEAR_FEATURE, 0, ep as u16)
}

/// Reset the device's BOT state machine. A pipe that did not halt makes
/// its controller-side reset fail; the device-side clear still runs. Both
/// pipes are cleared even when the device refuses the class reset (BOT 1.0,
/// 5.3.4, names the reset then both clears, as recalled): a halt left on
/// either side fails every command after it.
pub fn reset_recovery(disk: &Disk) -> Result<(), i32> {
    let _window = Window::open(RECOVERY_MS);
    let mut result =
        control_no_data(disk.xhci, disk.slot, MASS_STORAGE_RESET, 0, disk.interface as u16);
    for (dir_in, ep) in [(true, disk.ep_in), (false, disk.ep_out)] {
        let _ = reset_bulk(disk.xhci, disk.slot, dir_in);
        let cleared = control_no_data(disk.xhci, disk.slot, CLEAR_FEATURE, 0, ep as u16);
        if result.is_ok() {
            result = cleared;
        }
    }
    result
}

/// A command that lost its phase: the transport is reset, with time of its
/// own so one that ran out of time is reset too, and the failure named,
/// `E_TIMEDOUT` when the time ran out, else `E_IO`. It is heard as an
/// attention (`Said`): a reader clears its unit attention once it fails a
/// command with CHECK CONDITION for it, so a lost CSW, that command's or
/// that of the REQUEST SENSE that read it, may take a medium change with it.
pub(super) fn lost(disk: &Disk, state: &mut State, e: i32) -> i32 {
    state.heard(Said::Attention);
    let _ = reset_recovery(disk);
    if e == E_TIMEDOUT {
        E_TIMEDOUT
    } else {
        E_IO
    }
}
