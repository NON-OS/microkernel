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

//! Transfers on the bulk pipes driver.xhci0 keeps for a slot.

use super::call::call;
use super::wire::{BULK_MAX, DATA_AT, E_INVAL, E_IO, OP_BULK_IN_POLL, OP_BULK_OUT};

fn moved(resp: &[u8], len: usize) -> Result<usize, i32> {
    if len < 4 {
        return Err(E_IO);
    }
    Ok(u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize)
}

/// Send `data`, 1 to BULK_MAX bytes; any other length is refused before
/// it is sent. Returns the bytes the device took.
pub fn bulk_out(xhci: u32, slot: u8, data: &[u8]) -> Result<usize, i32> {
    if data.is_empty() || data.len() > BULK_MAX {
        return Err(E_INVAL);
    }
    let mut body = [0u8; 4 + BULK_MAX];
    body[0] = slot;
    body[2..4].copy_from_slice(&(data.len() as u16).to_le_bytes());
    body[4..4 + data.len()].copy_from_slice(data);
    let mut resp = [0u8; DATA_AT + 4];
    let len = call(xhci, OP_BULK_OUT, &body[..4 + data.len()], &mut resp)?;
    moved(&resp, len)
}

/// One look at the polled IN pipe with `out.len()` bytes, at most
/// BULK_MAX, armed. `E_AGAIN` from the controller driver is `None`.
pub fn bulk_in_poll(xhci: u32, slot: u8, out: &mut [u8]) -> Result<Option<usize>, i32> {
    let want = (out.len().min(BULK_MAX) as u16).to_le_bytes();
    let mut resp = [0u8; DATA_AT + 4 + BULK_MAX];
    let len = match call(xhci, OP_BULK_IN_POLL, &[slot, 0, want[0], want[1]], &mut resp) {
        Ok(len) => len,
        Err(super::wire::E_AGAIN) => return Ok(None),
        Err(e) => return Err(e),
    };
    let n = moved(&resp, len)?;
    if n > out.len() || len < 4 + n {
        return Err(E_IO);
    }
    out[..n].copy_from_slice(&resp[DATA_AT + 4..DATA_AT + 4 + n]);
    Ok(Some(n))
}
