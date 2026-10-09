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

//! Control transfers on endpoint 0 of a slot.

use super::call::call;
use super::wire::{CONTROL_MAX, DATA_AT, E_INVAL, E_IO, OP_CONTROL_TRANSFER};
use crate::setup::Setup;

fn setup_body(slot: u8, s: Setup, len: usize) -> [u8; 10] {
    let mut body = [0u8; 10];
    body[0] = slot;
    body[2] = s.request_type;
    body[3] = s.request;
    body[4..6].copy_from_slice(&s.value.to_le_bytes());
    body[6..8].copy_from_slice(&s.index.to_le_bytes());
    body[8..10].copy_from_slice(&(len as u16).to_le_bytes());
    body
}

/// A data stage in of up to `out.len()` bytes, at most CONTROL_MAX.
pub fn control_in(xhci: u32, slot: u8, s: Setup, out: &mut [u8]) -> Result<usize, i32> {
    let want = out.len().min(CONTROL_MAX);
    let body = setup_body(slot, s, want);
    let mut resp = [0u8; DATA_AT + 2 + CONTROL_MAX];
    let len = call(xhci, OP_CONTROL_TRANSFER, &body, &mut resp)?;
    let actual = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if len < 2 + actual || actual > want {
        return Err(E_IO);
    }
    out[..actual].copy_from_slice(&resp[DATA_AT + 2..DATA_AT + 2 + actual]);
    Ok(actual)
}

/// `data`, at most CONTROL_MAX bytes, as the data stage out; none when it
/// is empty.
pub fn control_out(xhci: u32, slot: u8, s: Setup, data: &[u8]) -> Result<(), i32> {
    if data.len() > CONTROL_MAX {
        return Err(E_INVAL);
    }
    let mut body = [0u8; 10 + CONTROL_MAX];
    body[..10].copy_from_slice(&setup_body(slot, s, data.len()));
    body[10..10 + data.len()].copy_from_slice(data);
    let mut resp = [0u8; DATA_AT + 2 + CONTROL_MAX];
    call(xhci, OP_CONTROL_TRANSFER, &body[..10 + data.len()], &mut resp).map(|_| ())
}
