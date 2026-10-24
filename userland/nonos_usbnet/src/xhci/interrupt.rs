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
//! A function's notification endpoint on the host wire: its transfer ring
//! made (ALLOC_TRANSFER_RING, 0x09), then looked at (INTERRUPT_IN, 0x0E),
//! each look answered at once, -11 while nothing came.

use super::call::call;
use super::wire::{DATA_AT, E_AGAIN, E_IO};
use crate::bus::Notify;

const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
const OP_INTERRUPT_IN: u16 = 0x000E;
/// The most one look takes. The host arms a report at the endpoint's whole
/// payload; a notice longer than this is refused below, never cut.
pub const NOTIFY_MAX: usize = 64;

pub fn configure_interrupt(xhci: u32, slot: u8, ep: Notify) -> Result<u8, i32> {
    let mps = ep.max_packet.to_le_bytes();
    let mut resp = [0u8; DATA_AT + 4];
    call(
        xhci,
        OP_ALLOC_TRANSFER_RING,
        &[slot, ep.address, mps[0], mps[1], ep.interval, 0],
        &mut resp,
    )?;
    Ok(resp[DATA_AT])
}

pub fn interrupt_in_poll(
    xhci: u32,
    slot: u8,
    dci: u8,
    out: &mut [u8],
) -> Result<Option<usize>, i32> {
    let want = (out.len().min(NOTIFY_MAX) as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 2 + NOTIFY_MAX];
    let n = match call(xhci, OP_INTERRUPT_IN, &[slot, dci, want[0], want[1]], &mut resp) {
        Ok(n) => n,
        Err(E_AGAIN) => return Ok(None),
        Err(e) => return Err(e),
    };
    let got = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if n < 2 + got || got > out.len().min(NOTIFY_MAX) {
        return Err(E_IO);
    }
    out[..got].copy_from_slice(&resp[DATA_AT + 2..DATA_AT + 2 + got]);
    Ok(Some(got))
}
