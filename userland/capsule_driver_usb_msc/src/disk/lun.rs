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

//! GET MAX LUN (USB MSC BOT 1.0, section 3.2): how many logical units the
//! device has. A stick has one; a card reader has one per slot, and the
//! card is often not in the first. A device that stalls the request, as
//! the specification lets one with a single unit do, or answers past 15,
//! has one, as Linux's usb_stor_Bulk_max_lun decides.

use super::types::Disk;
use crate::xhci::control_in;

const GET_MAX_LUN: (u8, u8) = (0xA1, 0xFE);
const MAX_LUN: u8 = 15;

/// The highest LUN, 0 to 15.
pub fn max_lun(disk: &Disk) -> u8 {
    let mut answer = [0u8; 1];
    match control_in(disk.xhci, disk.slot, GET_MAX_LUN, 0, disk.interface as u16, &mut answer) {
        Ok(1) if answer[0] <= MAX_LUN => answer[0],
        _ => 0,
    }
}
