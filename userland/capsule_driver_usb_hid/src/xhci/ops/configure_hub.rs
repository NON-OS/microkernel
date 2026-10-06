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

//! Tell the controller a slot is a hub: the Hub flag, its port count and,
//! for a high-speed hub, the TT think time go into its Slot Context through
//! Configure Endpoint (xHCI 1.2 section 6.2.2, Linux
//! `xhci_update_hub_device`). The controller needs them to schedule split
//! transactions for low and full speed devices below the hub.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_CONFIGURE_HUB, STATUS_LEN};

/// MTT stays off: a multi-TT hub runs single-TT until its alternate
/// setting 1 is selected, which this driver does not do (USB 2.0 section
/// 11.23.1).
const MTT: u8 = 0;

pub fn configure_hub(
    xhci_port: u32,
    slot: u8,
    ports: u8,
    think_time: u8,
) -> Result<(), XhciClientError> {
    let mut resp = [0u8; HDR_LEN + STATUS_LEN];
    let (status, _) =
        call(xhci_port, OP_CONFIGURE_HUB, &[slot, ports, think_time, MTT], &mut resp)?;
    if status != 0 {
        return Err(XhciClientError::Status(status));
    }
    Ok(())
}
