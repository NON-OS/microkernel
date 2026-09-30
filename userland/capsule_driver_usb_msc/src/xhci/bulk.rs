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

//! The bulk pipes driver.xhci0 keeps for this driver's device.

use super::call::call;
use super::wire::{BULK_MAX, DATA_AT, OP_BULK_IN, OP_BULK_OUT, OP_CONFIGURE_BULK, OP_RESET_BULK};
use crate::descriptors::MscBinding;
use crate::protocol::E_IO;

pub fn configure_bulk(xhci: u32, slot: u8, b: &MscBinding) -> Result<(), i32> {
    let mut body = [slot, b.bulk_in, b.bulk_out, 0, 0, 0, 0, 0];
    body[4..6].copy_from_slice(&b.max_packet_in.to_le_bytes());
    body[6..8].copy_from_slice(&b.max_packet_out.to_le_bytes());
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_CONFIGURE_BULK, &body, &mut resp).map(|_| ())
}

fn moved(resp: &[u8], len: usize) -> Result<usize, i32> {
    if len < 4 {
        return Err(E_IO);
    }
    Ok(u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize)
}

/// Send `data`, 1 to BULK_MAX bytes. Returns the bytes the device took.
pub fn bulk_out(xhci: u32, slot: u8, data: &[u8]) -> Result<usize, i32> {
    let mut body = [0u8; 4 + BULK_MAX];
    body[0] = slot;
    body[2..4].copy_from_slice(&(data.len() as u16).to_le_bytes());
    body[4..4 + data.len()].copy_from_slice(data);
    let mut resp = [0u8; DATA_AT + 4];
    let len = call(xhci, OP_BULK_OUT, &body[..4 + data.len()], &mut resp)?;
    moved(&resp, len)
}

/// Receive up to `out.len()`, at most BULK_MAX, bytes. Returns how many came.
pub fn bulk_in(xhci: u32, slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    let want = (out.len() as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 4 + BULK_MAX];
    let len = call(xhci, OP_BULK_IN, &[slot, 0, want[0], want[1]], &mut resp)?;
    let n = moved(&resp, len)?;
    if n > out.len() || len < 4 + n {
        return Err(E_IO);
    }
    out[..n].copy_from_slice(&resp[DATA_AT + 4..DATA_AT + 4 + n]);
    Ok(n)
}

/// Bring a stalled pipe back on the controller side.
pub fn reset_bulk(xhci: u32, slot: u8, dir_in: bool) -> Result<(), i32> {
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_RESET_BULK, &[slot, dir_in as u8], &mut resp).map(|_| ())
}
