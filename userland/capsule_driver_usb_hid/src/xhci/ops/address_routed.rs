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

//! Address Device for a device below a hub. The root port is not reset:
//! the hub already reset its own port, and resetting the root port would
//! reset the hub. The controller driver writes the route string, the speed
//! the hub reported and the TT into the Slot Context (xHCI 1.2 section
//! 4.3.3 and 6.2.2).

use crate::hub::Place;
use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{E_BUSY, HDR_LEN, OP_ADDRESS_ROUTED, STATUS_LEN};

use super::address_device::AddressedDevice;

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_LEN: usize = 8;

pub fn address_routed(
    xhci_port: u32,
    slot: u8,
    place: &Place,
) -> Result<AddressedDevice, XhciClientError> {
    let mut body = [0u8; 9];
    body[0] = slot;
    body[1] = place.root_port;
    body[2] = place.speed;
    body[3..7].copy_from_slice(&place.route.to_le_bytes());
    if let Some(tt) = place.tt {
        body[7] = tt.hub_slot;
        body[8] = tt.port;
    }
    let mut resp = [0u8; HDR_AND_STATUS + REPLY_LEN];
    let (status, data_len) = call(xhci_port, OP_ADDRESS_ROUTED, &body, &mut resp)?;
    if status == E_BUSY {
        return Err(XhciClientError::Busy);
    }
    if status != 0 {
        return Err(XhciClientError::Status(status));
    }
    if data_len < REPLY_LEN {
        return Err(XhciClientError::BadResponse);
    }
    let o = HDR_AND_STATUS;
    Ok(AddressedDevice {
        slot_id: resp[o],
        port_id: resp[o + 1],
        speed: resp[o + 2],
        max_packet_size: u16::from_le_bytes([resp[o + 4], resp[o + 5]]),
    })
}
