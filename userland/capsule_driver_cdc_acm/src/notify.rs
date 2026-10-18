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
//! The notification endpoint on the host wire: its transfer ring made,
//! then polled, each look answered at once (-11 while nothing came).

use nonos_usbnet::xhci::{call, E_AGAIN, E_IO};

use crate::acm::Notify;

const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
const OP_INTERRUPT_IN: u16 = 0x000E;
/// A reply's data: after its 20-byte header and its status word.
const DATA_AT: usize = 24;
/// SERIAL_STATE is ten bytes. The host arms a report at the endpoint's whole
/// payload; a notice longer than this is refused below, never cut.
pub const NOTICE_MAX: usize = 64;

/// The endpoint's DCI on the host.
pub fn ring(xhci: u32, slot: u8, n: &Notify) -> Result<u8, i32> {
    let mps = n.max_packet.to_le_bytes();
    let body = [slot, n.address, mps[0], mps[1], n.interval, 0];
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_ALLOC_TRANSFER_RING, &body, &mut resp)?;
    Ok(resp[DATA_AT])
}

/// One look: the notification that came, if any, in `out`.
pub fn poll(
    xhci: u32,
    slot: u8,
    dci: u8,
    out: &mut [u8; NOTICE_MAX],
) -> Result<Option<usize>, i32> {
    let len = (NOTICE_MAX as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 2 + NOTICE_MAX];
    let n = match call(xhci, OP_INTERRUPT_IN, &[slot, dci, len[0], len[1]], &mut resp) {
        Ok(n) => n,
        Err(E_AGAIN) => return Ok(None),
        Err(e) => return Err(e),
    };
    let actual = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if n < 2 + actual || actual > NOTICE_MAX {
        return Err(E_IO);
    }
    out[..actual].copy_from_slice(&resp[DATA_AT + 2..DATA_AT + 2 + actual]);
    Ok(Some(actual))
}
