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

//! The isoch ops of driver.xhci0 (`capsule_driver_xhci/src/protocol`):
//! configure the streaming endpoint, then send it batches of packets.

use nonos_usbnet::xhci::{call, E_IO};

use crate::uac::Stream;

const OP_CONFIGURE_ISOCH: u16 = 0x0018;
const OP_ISOCH_OUT: u16 = 0x0019;
/// A reply's data: after its 20-byte header and its status word.
pub const DATA_AT: usize = 24;
/// The most one request carries after the 4-byte head: 32 packet lengths.
const PACKETS_MAX: usize = 32;

/// The endpoint's DCI. Full and high speed only: no SuperSpeed companion
/// is read, so its bursts are none.
pub fn configure_isoch(xhci: u32, slot: u8, p: &Stream) -> Result<u8, i32> {
    let mps = p.max_packet.to_le_bytes();
    let body = [slot, p.ep_address, p.b_interval, 0, mps[0], mps[1], 0, 0];
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_CONFIGURE_ISOCH, &body, &mut resp)?;
    Ok(resp[DATA_AT])
}

/// `packets` packets of `packet` bytes each, `data` their bytes. Returns
/// the completion code of the batch sent before: 1 played, 23 late.
pub fn isoch_out(xhci: u32, slot: u8, packet: u16, packets: u8, data: &[u8]) -> Result<u8, i32> {
    let n = packets as usize;
    if n == 0 || n > PACKETS_MAX || data.len() != n * packet as usize {
        return Err(E_IO);
    }
    let mut body = alloc::vec![slot, 0, packets, 0];
    for _ in 0..n {
        body.extend_from_slice(&packet.to_le_bytes());
    }
    body.extend_from_slice(data);
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_ISOCH_OUT, &body, &mut resp)?;
    Ok(resp[DATA_AT])
}
