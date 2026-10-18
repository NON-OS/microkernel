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

//! The two driver.xhci0 operations nonos_usbnet does not wrap: adding an
//! interrupt IN endpoint to a slot, and one look at it. The layouts are
//! those of capsule_driver_xhci's `alloc_transfer_ring` and `interrupt_in`
//! handlers. B-43 is what lets the interrupt endpoint share a slot with
//! the bulk pair and read more than 8 octets; without it the second
//! endpoint configured is refused.

use super::call::call;
use super::limit::{reply_len, INTERRUPT_MAX};
use super::wire::{DATA_AT, E_AGAIN, E_IO};
use nonos_bt_hci::usb::Endpoint;

const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
const OP_INTERRUPT_IN: u16 = 0x000E;

/// Add `ep` to the slot: slot, bEndpointAddress, wMaxPacketSize and
/// bInterval. The answer is the device context index driver.xhci0 gave it,
/// which every later read names.
pub fn configure_interrupt(xhci: u32, slot: u8, ep: &Endpoint) -> Result<u8, i32> {
    let [m0, m1] = ep.max_packet.to_le_bytes();
    let body = [slot, ep.address, m0, m1, ep.interval, 0];
    let mut resp = [0u8; DATA_AT + 4];
    let len = call(xhci, OP_ALLOC_TRANSFER_RING, &body, &mut resp)?;
    // DCI 0 is the slot context and 1 the default endpoint (xHCI 1.2,
    // 4.5.1): neither can be an interrupt IN endpoint.
    match resp[DATA_AT] {
        dci if len >= 4 && dci >= 2 => Ok(dci),
        _ => Err(E_IO),
    }
}

/// One look at the endpoint with `out.len()` octets armed, at most
/// INTERRUPT_MAX; `None` while the device has sent nothing. driver.xhci0
/// keeps the transfer armed between looks, so nothing sent is lost.
pub fn interrupt_poll(xhci: u32, slot: u8, dci: u8, out: &mut [u8]) -> Result<Option<usize>, i32> {
    let armed = out.len().min(INTERRUPT_MAX);
    let want = (armed as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 2 + INTERRUPT_MAX];
    let len = match call(xhci, OP_INTERRUPT_IN, &[slot, dci, want[0], want[1]], &mut resp) {
        Ok(len) => len,
        Err(E_AGAIN) => return Ok(None),
        Err(e) => return Err(e),
    };
    let claimed = usize::from(u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]));
    let n = reply_len(armed, claimed, len).ok_or(E_IO)?;
    out[..n].copy_from_slice(&resp[DATA_AT + 2..DATA_AT + 2 + n]);
    Ok(Some(n))
}
